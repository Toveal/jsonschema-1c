use fluent_uri::error::ParseError;
use std::error::Error;

#[derive(Debug)]
pub enum JsonSchema1CError {
    SchemaCompile { msg: String },
    SchemeNotInstalled,
    StringConversionError { n_param: u32 },
    PropertyIdNotFound,
    UriConversionError { msg: ParseError<String> },
    PropertyIdNotString,
    JsonReadError { msg: serde_json::Error },
    ParamUnpackError,
}

impl Error for JsonSchema1CError {}

impl std::fmt::Display for JsonSchema1CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonSchema1CError::SchemaCompile { msg } => {
                write!(f, "Scheme compilation error: {msg}")
            }
            JsonSchema1CError::SchemeNotInstalled => write!(f, "Scheme not installed"),
            JsonSchema1CError::StringConversionError { n_param } => {
                write!(f, "Error converting parameter {n_param} to a string")
            }
            JsonSchema1CError::PropertyIdNotFound => {
                write!(f, "Property '$id' not found in the schema")
            }
            JsonSchema1CError::UriConversionError { msg } => {
                write!(f, "Failed to convert id to url: {msg}")
            }
            JsonSchema1CError::PropertyIdNotString => write!(f, "Property '$id' is not a string"),
            JsonSchema1CError::JsonReadError { msg } => write!(f, "JSON reading error: {msg}"),
            JsonSchema1CError::ParamUnpackError => {
                write!(f, "Internal component error. Expected parameter not found")
            }
        }
    }
}

impl From<serde_json::Error> for JsonSchema1CError {
    fn from(value: serde_json::Error) -> Self {
        JsonSchema1CError::JsonReadError { msg: value }
    }
}

impl<'a> From<jsonschema::ValidationError<'a>> for JsonSchema1CError {
    fn from(value: jsonschema::ValidationError) -> Self {
        JsonSchema1CError::SchemaCompile {
            msg: format!("{} {}", value.instance_path, value),
        }
    }
}

impl From<ParseError<String>> for JsonSchema1CError {
    fn from(value: ParseError<String>) -> Self {
        JsonSchema1CError::UriConversionError { msg: value }
    }
}
