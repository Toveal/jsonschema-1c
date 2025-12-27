use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParamType {
    String,
    Bool,
    Uri,
}

impl Display for ParamType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamType::String => write!(f, "string"),
            ParamType::Bool => write!(f, "bool"),
            ParamType::Uri => write!(f, "uri"),
        }
    }
}

#[derive(Debug)]
pub enum JsonSchema1CError {
    SchemaCompile { msg: String },
    SchemeNotInstalled,
    PropertyIdNotFound,
    PropertyIdNotString,
    JsonReadError { msg: serde_json::Error },
    OutOfMemory,
    ParamNotFound(usize),
    ConvertParamType { num: usize, p_type: ParamType },
    PropertyConvertType(ParamType),
}

impl Error for JsonSchema1CError {}

impl Display for JsonSchema1CError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonSchema1CError::SchemaCompile { msg } => {
                write!(f, "Scheme compilation error: {msg}")
            }
            JsonSchema1CError::SchemeNotInstalled => write!(f, "Scheme not installed"),
            JsonSchema1CError::PropertyIdNotFound => {
                write!(f, "Property '$id' not found in the schema")
            }
            JsonSchema1CError::PropertyIdNotString => write!(f, "Property '$id' is not a string"),
            JsonSchema1CError::JsonReadError { msg } => write!(f, "JSON reading error: {msg}"),
            JsonSchema1CError::OutOfMemory => write!(f, "Out of memory"),
            JsonSchema1CError::ConvertParamType { num, p_type } => {
                write!(f, "Failed to extract parameter {num} as '{p_type}'")
            }
            JsonSchema1CError::ParamNotFound(num) => write!(f, "Param '{}' not found", num),
            JsonSchema1CError::PropertyConvertType(t) => {
                write!(f, "Failed to extract property as '{t}'")
            }
        }
    }
}

impl From<serde_json::Error> for JsonSchema1CError {
    fn from(value: serde_json::Error) -> Self {
        JsonSchema1CError::JsonReadError { msg: value }
    }
}

impl From<jsonschema::ValidationError<'_>> for JsonSchema1CError {
    fn from(value: jsonschema::ValidationError) -> Self {
        JsonSchema1CError::SchemaCompile {
            msg: format!("{} {}", value.instance_path(), value),
        }
    }
}
