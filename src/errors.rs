use std::error::Error;

#[derive(Debug)]
pub enum JsonSchema1CError {
    SchemaCompile,
    SchemeNotInstalled,
    StringConversionError { n_param: u32 },
    PropertyIdNotFound,
    UrlConversionError,
    PropertyIdNotString,
    BooleanConverstionError { n_param: u32 },
}

impl Error for JsonSchema1CError {}

impl std::fmt::Display for JsonSchema1CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonSchema1CError::SchemaCompile => write!(f, "Scheme compilation error"),
            JsonSchema1CError::SchemeNotInstalled => write!(f, "Scheme not installed"),
            JsonSchema1CError::StringConversionError { n_param } => {
                write!(f, "Error converting parameter {n_param} to a string")
            }
            JsonSchema1CError::PropertyIdNotFound => {
                write!(f, "Property '$id' not found in the schema")
            }
            JsonSchema1CError::UrlConversionError => write!(f, "Failed to convert id to url"),
            JsonSchema1CError::PropertyIdNotString => write!(f, "Property '$id' is not a string"),
            JsonSchema1CError::BooleanConverstionError { n_param } => {
                write!(f, "Error converting parameter {n_param} to boolean")
            }
        }
    }
}
