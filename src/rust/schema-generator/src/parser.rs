use std::fs;
use std::path::{Path, PathBuf};

use crate::model::{Entity, InverseProp, Prop, Schema, TypeDef};
use crate::Result;

pub fn load_schemas(schema_dir: &Path) -> Result<Vec<Schema>> {
    let mut schema_files: Vec<PathBuf> = fs::read_dir(schema_dir)
        .map_err(|err| format!("read schema dir {}: {err}", schema_dir.display()))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|ext| ext == "exp").unwrap_or(false))
        .collect();

    schema_files.sort();

    let mut schemas = Vec::new();
    for path in schema_files {
        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default()
            .to_string();
        let name_clean = name.replace('.', "_");
        let data = fs::read_to_string(&path)
            .map_err(|err| format!("read schema file {}: {err}", path.display()))?;
        let (entities, types) = parse_elements(&data);
        let entities = sort_entities(entities);
        let entities_snapshot = entities.clone();
        let mut entities = entities;
        for entity in &mut entities {
            walk_parents(entity, &entities_snapshot);
        }
        let entities = find_subclasses(entities);
        schemas.push(Schema {
            name,
            name_clean,
            entities,
            types,
        });
    }

    Ok(schemas)
}

fn exp_type_to_rust_type(exp_type_name: &str) -> String {
    match exp_type_name {
        "REAL" | "NUMBER" | "INTEGER" | "BINARY" => "number".to_string(),
        "STRING" => "string".to_string(),
        "BOOLEAN" => "boolean".to_string(),
        "LOGICAL" => "logical".to_string(),
        _ => exp_type_name.to_string(),
    }
}

fn exp_type_to_type_num(exp_type_name: &str) -> u32 {
    match exp_type_name {
        "INTEGER" => 10,
        "REAL" | "NUMBER" | "BINARY" => 4,
        "STRING" => 1,
        "BOOLEAN" | "LOGICAL" => 3,
        _ => 5,
    }
}

fn parse_inverse(line: &str, entity: &mut Entity) {
    let split: Vec<&str> = line.split_whitespace().collect();
    if split.is_empty() {
        return;
    }
    let name = split[0].replace("INVERSE", "").trim().to_string();
    let set = split.iter().any(|val| *val == "SET" || *val == "LIST");
    if split.len() < 3 {
        return;
    }
    let for_val = split.last().unwrap_or(&"").replace(';', "");
    let type_name = split[split.len() - 3];
    let ts_type = exp_type_to_rust_type(type_name);
    entity.inverse_props.push(InverseProp {
        name,
        type_name: ts_type,
        set,
        for_ref: for_val,
    });
}

fn parse_derived(line: &str, entity: &mut Entity) {
    let trimmed = line.replace("DERIVE", "").trim().to_string();
    let chunks: Vec<&str> = trimmed.split_whitespace().collect();
    if let Some(first) = chunks.first() {
        if first.trim().starts_with("SELF") {
            let split: Vec<&str> = first.split('.').collect();
            if split.len() > 1 {
                entity.ifc_derived_props.push(split[1].to_string());
            }
        }
    }
}

fn parse_elements(data: &str) -> (Vec<Entity>, Vec<TypeDef>) {
    let mut entities = Vec::new();
    let mut types = Vec::new();
    let mut current_type: Option<TypeDef> = None;
    let mut current_entity: Option<Entity> = None;
    let mut read_props = false;
    let mut read_inverse = false;
    let mut read_ifc_derived = false;

    for raw_line in data.split(';') {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let has_colon = line.contains(" : ");

        if line.starts_with("ENTITY") {
            let split: Vec<&str> = line.split_whitespace().collect();
            let name = split.get(1).unwrap_or(&"").trim().to_string();
            let mut entity = Entity {
                name: name.clone(),
                parent: None,
                props: Vec::new(),
                children: Vec::new(),
                derived_props: Vec::new(),
                inverse_props: Vec::new(),
                derived_inverse_props: Vec::new(),
                is_ifc_product: name == "IfcProduct",
                ifc_derived_props: Vec::new(),
            };
            read_props = true;
            read_inverse = false;
            read_ifc_derived = false;

            if let Some(index) = split.iter().position(|val| *val == "SUBTYPE") {
                if let Some(parent_raw) = split.get(index + 2) {
                    let parent = parent_raw.replace('(', "").replace(')', "");
                    entity.parent = Some(parent);
                }
            }

            current_entity = Some(entity);
        } else if line.starts_with("END_ENTITY") {
            if let Some(entity) = current_entity.take() {
                entities.push(entity);
            }
            read_props = false;
            read_inverse = false;
            read_ifc_derived = false;
        } else if line.starts_with("WHERE") {
            read_props = false;
            read_inverse = false;
            read_ifc_derived = false;
        } else if line.starts_with("INVERSE") {
            read_props = false;
            read_inverse = true;
            read_ifc_derived = false;
            if let Some(entity) = current_entity.as_mut() {
                parse_inverse(line, entity);
            }
        } else if line.starts_with("DERIVE") {
            read_props = false;
            read_inverse = false;
            read_ifc_derived = true;
            if let Some(entity) = current_entity.as_mut() {
                parse_derived(line, entity);
            }
        } else if line.starts_with("UNIQUE") {
            read_props = false;
            read_inverse = false;
            read_ifc_derived = false;
        } else if line.starts_with("TYPE") {
            read_props = false;
            read_inverse = false;
            read_ifc_derived = false;

            let split: Vec<&str> = line.split_whitespace().map(|s| s.trim()).collect();
            let name = split.get(1).unwrap_or(&"").to_string();
            let is_list = split
                .iter()
                .any(|val| *val == "LIST" || *val == "SET" || *val == "ARRAY");
            let is_enum = split.iter().any(|val| *val == "ENUMERATION");
            let is_select = split
                .get(3)
                .map(|val| val.starts_with("SELECT"))
                .unwrap_or(false);
            let mut values = Vec::new();
            let mut type_name = String::new();

            if is_list {
                if let Some(last) = split.last() {
                    type_name = last.to_string();
                }
            } else if is_enum || is_select {
                if let Some(first_bracket) = line.find('(') {
                    if let Some(second_bracket) = line[first_bracket + 1..].find(')') {
                        let end = first_bracket + 1 + second_bracket;
                        let string_list = &line[first_bracket + 1..end];
                        values = string_list
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }
            } else if let Some(type_raw) = split.get(3) {
                type_name = type_raw.to_string();
            }

            if let Some(first_bracket) = type_name.find('(') {
                type_name = type_name[..first_bracket].to_string();
            }

            let type_num = exp_type_to_type_num(&type_name);
            let type_name = exp_type_to_rust_type(&type_name);

            current_type = Some(TypeDef {
                name,
                type_name,
                type_num,
                is_list,
                is_enum,
                is_select,
                values,
            });
        } else if line.starts_with("END_TYPE") {
            if let Some(ty) = current_type.take() {
                types.push(ty);
            }
        } else if read_inverse && has_colon {
            if let Some(entity) = current_entity.as_mut() {
                parse_inverse(line, entity);
            }
        } else if read_ifc_derived && has_colon {
            if let Some(entity) = current_entity.as_mut() {
                parse_derived(line, entity);
            }
        } else if read_props && has_colon {
            if let Some(entity) = current_entity.as_mut() {
                let split: Vec<&str> = line.split_whitespace().collect();
                let name = split.get(0).unwrap_or(&"").to_string();
                let mut optional = split.iter().any(|val| *val == "OPTIONAL");
                let set = split.iter().any(|val| *val == "SET" || *val == "LIST");
                let mut dimensions = 0usize;

                if set && !optional {
                    let set_loc = split
                        .iter()
                        .position(|val| *val == "SET" || *val == "LIST")
                        .unwrap_or(0);
                    if let Some(range) = split.get(set_loc + 1) {
                        if range.contains("[0:") {
                            optional = true;
                        }
                    }
                }

                if set {
                    dimensions = line.matches("LIST").count();
                }

                let mut type_name = split.last().unwrap_or(&"").replace(';', "");
                if let Some(first_bracket) = type_name.find('(') {
                    type_name = type_name[..first_bracket].to_string();
                }
                let ts_type = exp_type_to_rust_type(&type_name);
                entity.props.push(Prop {
                    name,
                    type_name: ts_type.clone(),
                    type_num: exp_type_to_type_num(&type_name),
                    primitive: ts_type != type_name,
                    optional,
                    set,
                    dimensions,
                });
            }
        } else if read_ifc_derived && has_colon {
            if let Some(entity) = current_entity.as_mut() {
                parse_derived(line, entity);
            }
        }
    }

    (entities, types)
}

fn find_entity<'a>(name: &Option<String>, entities: &'a [Entity]) -> Option<&'a Entity> {
    let name = name.as_ref()?;
    entities.iter().find(|entity| entity.name == *name)
}

fn find_entity_mut<'a>(name: &str, entities: &'a mut [Entity]) -> Option<&'a mut Entity> {
    entities.iter_mut().find(|entity| entity.name == name)
}

fn find_subclasses(mut entities: Vec<Entity>) -> Vec<Entity> {
    let mut updates: Vec<(String, Vec<String>)> = Vec::new();
    for entity in entities.iter().rev() {
        if let Some(parent_name) = &entity.parent {
            let mut children = entity.children.clone();
            children.push(entity.name.clone());
            updates.push((parent_name.clone(), children));
        }
    }

    for (parent_name, children) in updates {
        if let Some(parent) = find_entity_mut(&parent_name, &mut entities) {
            parent.children.extend(children);
        }
    }

    entities
}

fn walk_parents(entity: &mut Entity, entities: &[Entity]) {
    let parent = find_entity(&entity.parent, entities);
    if let Some(parent) = parent {
        let mut parent_clone = parent.clone();
        walk_parents(&mut parent_clone, entities);
        if parent_clone.is_ifc_product {
            entity.is_ifc_product = true;
        }
        entity.derived_props = parent_clone
            .derived_props
            .into_iter()
            .chain(entity.props.clone())
            .collect();
        entity.derived_inverse_props = parent_clone
            .derived_inverse_props
            .into_iter()
            .chain(entity.inverse_props.clone())
            .collect();
    } else {
        entity.derived_props = entity.props.clone();
        entity.derived_inverse_props = entity.inverse_props.clone();
    }
}

fn sort_entities(entities: Vec<Entity>) -> Vec<Entity> {
    let mut sorted = Vec::new();
    let mut unsorted = entities;
    while !unsorted.is_empty() {
        for entity in &unsorted {
            if entity.parent.is_none()
                || sorted.iter().any(|sorted_entity: &Entity| {
                    entity.parent.as_ref() == Some(&sorted_entity.name)
                })
            {
                sorted.push(entity.clone());
            }
        }
        unsorted.retain(|entity| {
            !sorted
                .iter()
                .any(|sorted_entity| sorted_entity.name == entity.name)
        });
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_entity() {
        let input = "ENTITY IfcRoot; GlobalId : IfcLabel; END_ENTITY;";
        let (entities, types) = parse_elements(input);
        assert_eq!(types.len(), 0);
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].name, "IfcRoot");
        assert_eq!(entities[0].props.len(), 1);
    }
}
