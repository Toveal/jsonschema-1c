use std::error::Error;

#[derive(Debug)]
pub enum JsonSchema1CError {
    SchemaCompile { msg: String },
    SchemeNotInstalled,
    StringConversionError { n_param: u32 },
    PropertyIdNotFound,
    UrlConversionError { msg: url::ParseError },
    PropertyIdNotString,
    JsonReadError { msg: serde_json::Error },
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
            JsonSchema1CError::UrlConversionError { msg } => {
                write!(f, "Failed to convert id to url: {msg}")
            }
            JsonSchema1CError::PropertyIdNotString => write!(f, "Property '$id' is not a string"),
            JsonSchema1CError::JsonReadError { msg } => write!(f, "JSON reading error: {msg}"),
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

impl From<url::ParseError> for JsonSchema1CError {
    fn from(value: url::ParseError) -> Self {
        JsonSchema1CError::UrlConversionError { msg: value }
    }
}
