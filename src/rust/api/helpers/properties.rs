//! Web-IFC Properties helper.

use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

use crate::api::ifc_schema::{
    IFCPROJECT, IFCRELAGGREGATES, IFCRELASSOCIATESMATERIAL, IFCRELCONTAINEDINSPATIALSTRUCTURE,
    IFCRELDEFINESBYPROPERTIES, IFCRELDEFINESBYTYPE,
};
use crate::api::value::{Map, Value};
use crate::api::web_ifc_api::{IfcAPIState, IfcApiError, IfcVector, Vector};

#[derive(Clone)]
struct PropNames {
    name: i32,
    relating: &'static str,
    related: &'static str,
    key: &'static str,
}

fn prop_names() -> HashMap<&'static str, PropNames> {
    HashMap::from([
        (
            "aggregates",
            PropNames {
                name: IFCRELAGGREGATES as i32,
                relating: "RelatingObject",
                related: "RelatedObjects",
                key: "children",
            },
        ),
        (
            "spatial",
            PropNames {
                name: IFCRELCONTAINEDINSPATIALSTRUCTURE as i32,
                relating: "RelatingStructure",
                related: "RelatedElements",
                key: "children",
            },
        ),
        (
            "psets",
            PropNames {
                name: IFCRELDEFINESBYPROPERTIES as i32,
                relating: "RelatingPropertyDefinition",
                related: "RelatedObjects",
                key: "IsDefinedBy",
            },
        ),
        (
            "materials",
            PropNames {
                name: IFCRELASSOCIATESMATERIAL as i32,
                relating: "RelatingMaterial",
                related: "RelatedObjects",
                key: "HasAssociations",
            },
        ),
        (
            "type",
            PropNames {
                name: IFCRELDEFINESBYTYPE as i32,
                relating: "RelatingType",
                related: "RelatedObjects",
                key: "IsDefinedBy",
            },
        ),
    ])
}

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub express_id: i32,
    pub type_name: String,
    pub children: Vec<Node>,
    pub properties: Option<Value>,
}

#[derive(Clone)]
pub struct Properties {
    state: Arc<Mutex<IfcAPIState>>,
}

impl Properties {
    pub fn new(state: Arc<Mutex<IfcAPIState>>) -> Self {
        Self { state }
    }

    pub async fn get_item_properties(
        &self,
        model_id: i32,
        id: i32,
        recursive: bool,
        inverse: bool,
    ) -> Result<Value, IfcApiError> {
        self.get_line(model_id, id, recursive, inverse, None)
    }

    pub async fn get_property_sets(
        &self,
        model_id: i32,
        element_id: i32,
        recursive: bool,
        include_type_properties: bool,
    ) -> Result<Vec<Value>, IfcApiError> {
        self.get_property_sets_inner(model_id, element_id, recursive, include_type_properties)
    }

    fn get_property_sets_inner(
        &self,
        model_id: i32,
        element_id: i32,
        recursive: bool,
        include_type_properties: bool,
    ) -> Result<Vec<Value>, IfcApiError> {
        let names = prop_names();
        if include_type_properties {
            let types = self.get_type_properties(model_id, element_id, false)?;
            let mut results = Vec::new();
            for entry in types {
                if let Some(id) = entry.get("expressID").and_then(Value::as_i64) {
                    let nested =
                        self.get_property_sets_inner(model_id, id as i32, recursive, false)?;
                    results.extend(nested);
                }
            }
            Ok(results)
        } else {
            self.get_related_properties(model_id, element_id, &names["psets"], recursive)
        }
    }

    pub async fn set_property_sets(
        &self,
        model_id: i32,
        element_id: Vec<i32>,
        pset_id: Vec<i32>,
    ) -> Result<bool, IfcApiError> {
        let names = prop_names();
        self.set_item_properties(model_id, element_id, pset_id, &names["psets"])
    }

    pub fn get_type_properties(
        &self,
        model_id: i32,
        element_id: i32,
        recursive: bool,
    ) -> Result<Vec<Value>, IfcApiError> {
        let names = prop_names();
        let schema = self.get_model_schema(model_id)?;
        if schema == "IFC2X3" {
            self.get_related_properties(model_id, element_id, &names["type"], recursive)
        } else {
            let mut props = names["type"].clone();
            props.key = "IsTypedBy";
            self.get_related_properties(model_id, element_id, &props, recursive)
        }
    }

    pub async fn get_materials_properties(
        &self,
        model_id: i32,
        element_id: i32,
        recursive: bool,
        include_type_materials: bool,
    ) -> Result<Vec<Value>, IfcApiError> {
        self.get_materials_properties_inner(model_id, element_id, recursive, include_type_materials)
    }

    fn get_materials_properties_inner(
        &self,
        model_id: i32,
        element_id: i32,
        recursive: bool,
        include_type_materials: bool,
    ) -> Result<Vec<Value>, IfcApiError> {
        let names = prop_names();
        if include_type_materials {
            let types = self.get_type_properties(model_id, element_id, false)?;
            let mut results = Vec::new();
            for entry in types {
                if let Some(id) = entry.get("expressID").and_then(Value::as_i64) {
                    let nested =
                        self.get_materials_properties_inner(model_id, id as i32, recursive, false)?;
                    results.extend(nested);
                }
            }
            Ok(results)
        } else {
            self.get_related_properties(model_id, element_id, &names["materials"], recursive)
        }
    }

    pub async fn set_materials_properties(
        &self,
        model_id: i32,
        element_id: Vec<i32>,
        material_id: Vec<i32>,
    ) -> Result<bool, IfcApiError> {
        let names = prop_names();
        self.set_item_properties(model_id, element_id, material_id, &names["materials"])
    }

    pub async fn get_spatial_structure(
        &self,
        model_id: i32,
        include_properties: bool,
    ) -> Result<Node, IfcApiError> {
        let chunks = self.get_spatial_tree_chunks(model_id)?;
        let line_ids = self.get_line_ids_with_type(model_id, IFCPROJECT as i32)?;
        let project_id = line_ids.get(0).copied().unwrap_or_default();
        let mut project = Node {
            express_id: project_id,
            type_name: "IFCPROJECT".to_string(),
            children: Vec::new(),
            properties: None,
        };
        self.get_spatial_node(model_id, &mut project, &chunks, include_properties)?;
        Ok(project)
    }

    fn get_related_properties(
        &self,
        model_id: i32,
        element_id: i32,
        props_name: &PropNames,
        recursive: bool,
    ) -> Result<Vec<Value>, IfcApiError> {
        let mut result = Vec::new();
        let rels = if element_id != 0 {
            let line = self.get_line(model_id, element_id, false, true, Some(props_name.key))?;
            line.get(props_name.key).cloned()
        } else {
            let vec = self.get_line_ids_with_type(model_id, props_name.name)?;
            let list: Vec<Value> = vec
                .0
                .iter()
                .map(|id| {
                    Value::Object(Map::from_iter([(
                        "value".to_string(),
                        Value::Number((*id).into()),
                    )]))
                })
                .collect();
            Some(Value::Array(list))
        };

        let mut rels = match rels {
            Some(Value::Array(values)) => values,
            Some(other) => vec![other],
            None => return Ok(result),
        };

        for rel in rels.drain(..) {
            let rel_id = extract_id(&rel);
            if rel_id.is_none() {
                continue;
            }
            let rel_line = self.get_line(model_id, rel_id.unwrap(), false, false, None)?;
            let prop_set_ids = rel_line.get(props_name.relating).cloned();
            let mut prop_set_ids = match prop_set_ids {
                Some(Value::Array(values)) => values,
                Some(other) => vec![other],
                None => continue,
            };
            for prop_set in prop_set_ids.drain(..) {
                if let Some(id) = extract_id(&prop_set) {
                    let line = self.get_line(model_id, id, recursive, false, None)?;
                    result.push(line);
                }
            }
        }
        Ok(result)
    }

    fn get_chunks(
        &self,
        model_id: i32,
        chunks: &mut HashMap<i32, Vec<i32>>,
        prop_names: &PropNames,
    ) -> Result<(), IfcApiError> {
        let relation = self.get_line_ids_with_type(model_id, prop_names.name)?;
        for id in relation.0 {
            let rel = self.get_line(model_id, id, false, false, None)?;
            self.save_chunk(chunks, prop_names, &rel);
        }
        Ok(())
    }

    fn get_spatial_node(
        &self,
        model_id: i32,
        node: &mut Node,
        tree_chunks: &HashMap<i32, Vec<i32>>,
        include_properties: bool,
    ) -> Result<(), IfcApiError> {
        let names = prop_names();
        self.get_children(
            model_id,
            node,
            tree_chunks,
            &names["aggregates"],
            include_properties,
        )?;
        self.get_children(
            model_id,
            node,
            tree_chunks,
            &names["spatial"],
            include_properties,
        )?;
        Ok(())
    }

    fn get_children(
        &self,
        model_id: i32,
        node: &mut Node,
        tree_chunks: &HashMap<i32, Vec<i32>>,
        _prop_names: &PropNames,
        include_properties: bool,
    ) -> Result<(), IfcApiError> {
        let children = match tree_chunks.get(&node.express_id) {
            Some(children) => children.clone(),
            None => return Ok(()),
        };
        let mut nodes = Vec::new();
        for child_id in children {
            let type_code = self.get_line_type(model_id, child_id)?;
            let mut child_node = Node {
                express_id: child_id,
                type_name: self.get_name_from_type_code(type_code),
                children: Vec::new(),
                properties: None,
            };
            if include_properties {
                let properties = self.get_line(model_id, child_id, false, false, None)?;
                child_node.properties = Some(properties);
            }
            self.get_spatial_node(model_id, &mut child_node, tree_chunks, include_properties)?;
            nodes.push(child_node);
        }
        node.children = nodes;
        Ok(())
    }

    fn get_spatial_tree_chunks(
        &self,
        model_id: i32,
    ) -> Result<HashMap<i32, Vec<i32>>, IfcApiError> {
        let names = prop_names();
        let mut tree_chunks = HashMap::new();
        self.get_chunks(model_id, &mut tree_chunks, &names["aggregates"])?;
        self.get_chunks(model_id, &mut tree_chunks, &names["spatial"])?;
        Ok(tree_chunks)
    }

    fn save_chunk(&self, chunks: &mut HashMap<i32, Vec<i32>>, prop_names: &PropNames, rel: &Value) {
        let relating = rel
            .get(prop_names.relating)
            .and_then(extract_id)
            .unwrap_or_default();
        let related = rel
            .get(prop_names.related)
            .and_then(|value| value.as_array())
            .map(|array| array.iter().filter_map(extract_id).collect::<Vec<_>>())
            .unwrap_or_default();
        chunks.entry(relating).or_default().extend(related);
    }

    fn set_item_properties(
        &self,
        model_id: i32,
        element_id: Vec<i32>,
        prop_id: Vec<i32>,
        props_name: &PropNames,
    ) -> Result<bool, IfcApiError> {
        let mut elements = Vec::new();
        for id in &element_id {
            let element = self.get_line(model_id, *id, false, true, None)?;
            if element.get(props_name.key).is_some() {
                elements.push(element);
            }
        }
        if elements.is_empty() {
            return Ok(false);
        }
        let relations = self.get_line_ids_with_type(model_id, props_name.name)?;
        let mut rels = Vec::new();
        for rel_id in relations.0 {
            let rel = self.get_line(model_id, rel_id, false, false, None)?;
            let relating = rel.get(props_name.relating).and_then(extract_id);
            if let Some(relating) = relating {
                if prop_id.contains(&relating) {
                    rels.push(rel);
                }
            }
        }
        for mut element in elements {
            for rel in &rels {
                let rel_id = rel
                    .get("expressID")
                    .and_then(Value::as_i64)
                    .unwrap_or_default();
                let mut list = element
                    .get(props_name.key)
                    .and_then(|value| value.as_array().cloned())
                    .unwrap_or_default();
                if !list
                    .iter()
                    .any(|entry| extract_id(entry) == Some(rel_id as i32))
                {
                    list.push(Value::Object(Map::from_iter([
                        ("type".to_string(), Value::Number(5.into())),
                        ("value".to_string(), Value::Number(rel_id.into())),
                    ])));
                }
                element
                    .as_object_mut()
                    .unwrap_or_else(|| panic!("element not object"))
                    .insert(props_name.key.to_string(), Value::Array(list));
            }
            self.write_line(model_id, element)?;
        }
        Ok(true)
    }

    fn get_line(
        &self,
        model_id: i32,
        express_id: i32,
        _recursive: bool,
        _inverse: bool,
        _inverse_prop_key: Option<&str>,
    ) -> Result<Value, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        model
            .lines
            .get(&express_id)
            .cloned()
            .ok_or(IfcApiError::LineNotFound(express_id))
    }

    fn get_line_ids_with_type(
        &self,
        model_id: i32,
        type_code: i32,
    ) -> Result<IfcVector<i32>, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        let mut ids: Vec<i32> = model
            .line_types
            .iter()
            .filter_map(|(id, t)| if *t == type_code { Some(*id) } else { None })
            .collect();
        ids.sort_unstable();
        Ok(IfcVector::new(ids))
    }

    fn get_line_type(&self, model_id: i32, express_id: i32) -> Result<i32, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        model
            .line_types
            .get(&express_id)
            .copied()
            .ok_or(IfcApiError::LineNotFound(express_id))
    }

    fn get_name_from_type_code(&self, type_code: i32) -> String {
        format!("{type_code}")
    }

    fn get_model_schema(&self, model_id: i32) -> Result<String, IfcApiError> {
        let state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        Ok(model.schema.clone())
    }

    fn write_line(&self, model_id: i32, line: Value) -> Result<(), IfcApiError> {
        let mut state = self.state.lock().expect("state lock");
        let model = state
            .models
            .get_mut(&model_id)
            .ok_or(IfcApiError::ModelNotFound(model_id))?;
        let express_id = line
            .get("expressID")
            .and_then(Value::as_i64)
            .ok_or(IfcApiError::InvalidInput("expressID"))?;
        model.lines.insert(express_id as i32, line);
        Ok(())
    }
}

fn extract_id(value: &Value) -> Option<i32> {
    match value {
        Value::Object(map) => map.get("value").and_then(Value::as_i64).map(|v| v as i32),
        Value::Number(num) => Some(*num as i32),
        _ => None,
    }
}
