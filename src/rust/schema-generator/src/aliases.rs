#[derive(Clone, Copy, Debug)]
pub struct SchemaAlias {
    pub schema_name: &'static str,
    pub alias: &'static str,
}

pub const SCHEMA_ALIASES: &[SchemaAlias] = &[
    SchemaAlias {
        schema_name: "IFC2X_FINAL",
        alias: "IFC2X3",
    },
    SchemaAlias {
        schema_name: "IFC4X1",
        alias: "IFC4X3",
    },
    SchemaAlias {
        schema_name: "IFC4X2",
        alias: "IFC4X3",
    },
    SchemaAlias {
        schema_name: "IFC4X3_RC3",
        alias: "IFC4X3",
    },
    SchemaAlias {
        schema_name: "IFC4X3_RC4",
        alias: "IFC4X3",
    },
    SchemaAlias {
        schema_name: "IFC4X3_RC1",
        alias: "IFC4X3",
    },
    SchemaAlias {
        schema_name: "IFC4X3_RC2",
        alias: "IFC4X3",
    },
    SchemaAlias {
        schema_name: "IFC4X3_ADD2",
        alias: "IFC4X3",
    },
    SchemaAlias {
        schema_name: "IFC4X3_ADD1",
        alias: "IFC4X3",
    },
];
