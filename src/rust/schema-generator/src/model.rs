#[derive(Clone, Debug)]
pub struct TypeDef {
    pub name: String,
    pub type_name: String,
    pub type_num: u32,
    pub is_list: bool,
    pub is_enum: bool,
    pub is_select: bool,
    pub values: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Prop {
    pub name: String,
    pub type_name: String,
    pub type_num: u32,
    pub primitive: bool,
    pub optional: bool,
    pub set: bool,
    pub dimensions: usize,
}

#[derive(Clone, Debug)]
pub struct InverseProp {
    pub name: String,
    pub type_name: String,
    pub set: bool,
    pub for_ref: String,
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub props: Vec<Prop>,
    pub inverse_props: Vec<InverseProp>,
    pub derived_props: Vec<Prop>,
    pub ifc_derived_props: Vec<String>,
    pub derived_inverse_props: Vec<InverseProp>,
    pub is_ifc_product: bool,
}

#[derive(Clone, Debug)]
pub struct Schema {
    pub name: String,
    pub name_clean: String,
    pub entities: Vec<Entity>,
    pub types: Vec<TypeDef>,
}
