use std::{fs::File, io::Read, path::PathBuf};

use serde::{self, Deserialize};

use crate::generator::Module;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum FieldTypeJsonDefinition {
    Defined {
        defined: String,
    },
    Primitive(String),
    Option {
        option: Box<FieldTypeJsonDefinition>,
    },
    Array {
        array: (Box<FieldTypeJsonDefinition>, usize),
    },
    Vec {
        vec: Box<FieldTypeJsonDefinition>,
    },
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct FieldJsonDefinition {
    pub name: String,
    pub docs: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub field_type: FieldTypeJsonDefinition,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum EnumVariantJsonDefinition {
    Struct {
        docs: Option<Vec<String>>,
        name: String,
        fields: Vec<FieldJsonDefinition>,
    },
    Tuple {
        docs: Option<Vec<String>>,
        name: String,
        #[serde(rename = "fields")]
        types: Vec<FieldTypeJsonDefinition>,
    },
    UnitLike {
        docs: Option<Vec<String>>,
        name: String,
    },
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum TypedefTypeJsonDefinition {
    Enum {
        docs: Option<Vec<String>>,
        variants: Vec<EnumVariantJsonDefinition>,
    },
    Struct {
        docs: Option<Vec<String>>,
        fields: Vec<FieldJsonDefinition>,
    },
}

impl TypedefTypeJsonDefinition {
    pub fn is_enum(&self) -> bool {
        match self {
            Self::Enum { .. } => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct { .. } => true,
            _ => false,
        }
    }

    pub fn fields(&self) -> Option<Vec<FieldJsonDefinition>> {
        match self {
            Self::Struct { fields, .. } => Some(fields.clone()),
            _ => None,
        }
    }

    pub fn variants(&self) -> Option<Vec<EnumVariantJsonDefinition>> {
        match self {
            Self::Enum { variants, .. } => Some(variants.clone()),
            _ => None,
        }
    }

    pub fn docs(&self) -> Option<Vec<String>> {
        match self {
            Self::Enum { docs, .. } => docs.clone(),
            Self::Struct { docs, .. } => docs.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TypeDefJsonDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub typedef_type: TypedefTypeJsonDefinition,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct InstructionAccountJsonDefinition {
    pub name: String,
    #[serde(rename = "isMut")]
    pub is_mut: bool,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstructionJsonDefinition {
    pub name: String,
    pub accounts: Vec<InstructionAccountJsonDefinition>,
    pub args: Vec<FieldJsonDefinition>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventJsonDefinition {
    pub name: String,
    pub fields: Vec<FieldJsonDefinition>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorJsonDefinition {
    pub code: u32,
    pub name: String,
    pub msg: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct IdlJsonDefinition {
    pub version: String,
    pub name: String,
    pub instructions: Vec<InstructionJsonDefinition>,
    pub accounts: Vec<TypeDefJsonDefinition>,
    pub types: Vec<TypeDefJsonDefinition>,
    pub events: Vec<EventJsonDefinition>,
    pub errors: Vec<ErrorJsonDefinition>,
}

impl IdlJsonDefinition {
    pub fn is_type(&self, name: &String) -> bool {
        self.types
            .iter()
            .find(|current| current.name == *name)
            .is_some()
    }

    pub fn is_account(&self, name: &String) -> bool {
        self.accounts
            .iter()
            .find(|current| current.name == *name)
            .is_some()
    }

    pub fn get_typedef_module(&self, name: &String) -> Option<Module> {
        if self.is_type(name) {
            Some(Module::Types)
        } else if self.is_account(name) {
            Some(Module::Accounts)
        } else {
            None
        }
    }

    pub fn read_idl(idl_path: &PathBuf) -> IdlJsonDefinition {
        let mut file = File::open(idl_path).expect("Failed to read IDL");

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("Failed to read IDL");

        serde_json::from_str::<IdlJsonDefinition>(&file_contents).expect("Invalid IDL contents")
    }
}

#[cfg(test)]
mod test {
    use crate::idl::{
        EnumVariantJsonDefinition, ErrorJsonDefinition, EventJsonDefinition, FieldJsonDefinition,
        FieldTypeJsonDefinition, InstructionAccountJsonDefinition, InstructionJsonDefinition,
        TypeDefJsonDefinition,
    };

    #[test]
    fn parse_typedef_struct() {
        let typedef_str = r#"
        {
            "name": "OrderParams",
            "type": {
              "kind": "struct",
              "fields": [
                {
                    "name": "orderType",
                    "docs": [
                        "Order type"
                    ],
                    "type": {
                        "defined": "OrderType"
                    }
                },
                {
                    "name": "userOrderId",
                    "type": "u8"
                },
                {
                    "name": "maxTs",
                    "type": {
                        "option": "i64"
                    }
                },
                {
                    "name": "padding",
                    "type": {
                        "array": [
                            "u8",
                            6
                        ]
                    }
                },
                {
                    "name": "padding1",
                    "type": {
                        "vec": {
                            "defined": "paddingStruct"
                        }
                    }
                }
              ]
            }
        }
        "#;

        let typedef = serde_json::from_str::<TypeDefJsonDefinition>(typedef_str);
        assert!(typedef.is_ok());
        let typedef = typedef.unwrap();

        assert_eq!(typedef.name, "OrderParams".to_owned());
        assert_eq!(typedef.typedef_type.is_struct(), true);

        let fields = typedef.typedef_type.fields().unwrap();

        assert_eq!(fields.len(), 5);
        assert_eq!(
            fields[0].field_type,
            FieldTypeJsonDefinition::Defined {
                defined: "OrderType".to_owned(),
            }
        );
        assert_eq!(
            fields[1].field_type,
            FieldTypeJsonDefinition::Primitive("u8".to_owned()),
        );
        assert_eq!(
            fields[2].field_type,
            FieldTypeJsonDefinition::Option {
                option: Box::new(FieldTypeJsonDefinition::Primitive("i64".to_owned())),
            },
        );
        assert_eq!(
            fields[3].field_type,
            FieldTypeJsonDefinition::Array {
                array: (
                    Box::new(FieldTypeJsonDefinition::Primitive("u8".to_owned())),
                    6
                ),
            },
        );
        assert_eq!(
            fields[4].field_type,
            FieldTypeJsonDefinition::Vec {
                vec: Box::new(FieldTypeJsonDefinition::Defined {
                    defined: "paddingStruct".to_owned(),
                })
            }
        );
    }

    #[test]
    fn parse_typedef_enum() {
        let typedef_str = r#"
        {
            "name": "ModifyOrderId",
            "type": {
                "kind": "enum",
                "variants": [
                    {
                        "name": "UserOrderId",
                        "fields": [
                            "u8"
                        ]
                    },
                    {
                        "name": "OrderId"
                    },
                    {
                        "name": "StructField",
                        "fields": [
                            {
                                "name": "side",
                                "type": {
                                    "defined": "Side"
                                }
                            }
                        ]
                    }
                ]
            }
        }
        "#;

        let typedef = serde_json::from_str::<TypeDefJsonDefinition>(typedef_str);
        assert!(typedef.is_ok());
        let typedef = typedef.unwrap();

        assert_eq!(typedef.name, "ModifyOrderId".to_owned());
        assert_eq!(typedef.typedef_type.is_enum(), true);

        let variants = typedef.typedef_type.variants().unwrap();

        assert_eq!(variants.len(), 3);
        assert_eq!(
            variants[0],
            EnumVariantJsonDefinition::Tuple {
                docs: None,
                name: "UserOrderId".to_owned(),
                types: vec![FieldTypeJsonDefinition::Primitive("u8".to_owned())]
            }
        );
        assert_eq!(
            variants[1],
            EnumVariantJsonDefinition::UnitLike {
                docs: None,
                name: "OrderId".to_owned()
            }
        );
        assert_eq!(
            variants[2],
            EnumVariantJsonDefinition::Struct {
                docs: None,
                name: "StructField".to_owned(),
                fields: vec![FieldJsonDefinition {
                    name: "side".to_owned(),
                    docs: None,
                    field_type: FieldTypeJsonDefinition::Defined {
                        defined: "Side".to_owned()
                    }
                }],
            }
        );
    }

    #[test]
    fn parse_instruction() {
        let instruction_str = r#"
        {
            "name": "initializeUser",
            "accounts": [
                {
                    "name": "user",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "userStats",
                    "isMut": true,
                    "isSigner": false
                }
            ],
            "args": [
                {
                    "name": "subAccountId",
                    "type": "u16"
                },
                {
                    "name": "name",
                    "type": {
                    "array": [
                            "u8",
                            32
                        ]
                    }
                }
            ]
        }
        "#;

        let instruction = serde_json::from_str::<InstructionJsonDefinition>(instruction_str);
        assert!(instruction.is_ok());
        let instruction = instruction.unwrap();

        assert_eq!(instruction.name, "initializeUser".to_owned());
        assert_eq!(instruction.accounts.len(), 2);
        assert_eq!(
            instruction.accounts[0],
            InstructionAccountJsonDefinition {
                name: "user".to_owned(),
                is_mut: true,
                is_signer: false
            }
        );
        assert_eq!(instruction.args.len(), 2);
        assert_eq!(
            instruction.args[1],
            FieldJsonDefinition {
                name: "name".to_owned(),
                docs: None,
                field_type: FieldTypeJsonDefinition::Array {
                    array: (
                        Box::new(FieldTypeJsonDefinition::Primitive("u8".to_owned())),
                        32
                    )
                }
            }
        );
    }

    #[test]
    fn parse_event() {
        let event_str = r#"
        {
            "name": "NewUserRecord",
            "fields": [
                {
                    "name": "ts",
                    "type": "i64",
                    "index": false
                },
                {
                    "name": "name",
                    "type": {
                    "array": [
                            "u8",
                            32
                       ]
                    },
                    "index": false
                }
            ]
        }
        "#;

        let event = serde_json::from_str::<EventJsonDefinition>(event_str);
        assert!(event.is_ok());
        let event = event.unwrap();

        assert_eq!(event.name, "NewUserRecord".to_owned());
        assert_eq!(event.fields.len(), 2);
    }

    #[test]
    fn parse_error() {
        let error_str = r#"
        {
            "code": 6000,
            "name": "InvalidSpotMarketAuthority",
            "msg": "Invalid Spot Market Authority"
        }
        "#;

        let error = serde_json::from_str::<ErrorJsonDefinition>(error_str);
        assert!(error.is_ok());
        let error = error.unwrap();

        assert_eq!(error.code, 6000);
        assert_eq!(error.name, "InvalidSpotMarketAuthority".to_owned());
        assert_eq!(error.msg, "Invalid Spot Market Authority".to_owned());
    }
}
