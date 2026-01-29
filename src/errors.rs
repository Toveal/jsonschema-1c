use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParamType {
    String,
    Bool,
    Uri,
    Json,
    StringOrBlob,
}

impl Display for ParamType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::String => "string",
            Self::Bool => "bool",
            Self::Uri => "uri",
            Self::Json => "json",
            Self::StringOrBlob => "string or binary",
        };
        f.write_str(name)
    }
}

#[derive(Debug)]
pub enum JsonSchema1CError {
    // Schema errors
    SchemaCompile(String),
    SchemaNotInstalled,

    // Schema property errors
    PropertyIdNotFound,
    PropertyIdNotString,

    // Parameter errors
    ParamNotFound(usize),
    ParamConvert { index: usize, expected: ParamType },
    PropertyConvert(ParamType),

    // Other errors
    JsonParse(serde_json::Error),
    InvalidUri(String),
    OutOfMemory,
    UnknownDraft,
}

impl Error for JsonSchema1CError {}

impl Display for JsonSchema1CError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaCompile(msg) => write!(f, "Schema compilation error: {msg}"),
            Self::SchemaNotInstalled => f.write_str("Schema not installed"),
            Self::PropertyIdNotFound => f.write_str("Property '$id' not found in schema"),
            Self::PropertyIdNotString => f.write_str("Property '$id' is not a string"),
            Self::ParamNotFound(index) => write!(f, "Parameter {index} not found"),
            Self::ParamConvert { index, expected } => {
                write!(f, "Cannot convert parameter {index} to {expected}")
            }
            Self::PropertyConvert(expected) => {
                write!(f, "Cannot convert property to {expected}")
            }
            Self::JsonParse(e) => write!(f, "JSON parse error: {e}"),
            Self::OutOfMemory => f.write_str("Out of memory"),
            Self::InvalidUri(uri) => write!(f, "Invalid URI: {uri}"),
            Self::UnknownDraft => f.write_str("Unknown draft"),
        }
    }
}

impl From<serde_json::Error> for JsonSchema1CError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonParse(value)
    }
}

impl From<jsonschema::ValidationError<'_>> for JsonSchema1CError {
    fn from(err: jsonschema::ValidationError) -> Self {
        Self::SchemaCompile(format!("{} {}", err.instance_path(), err))
    }
}
