use jsonschema::Retrieve;
use jsonschema::Uri;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct RetrieveHandler {
    store: HashMap<Uri<String>, Value>,
}

impl RetrieveHandler {
    pub fn new(schemas: HashMap<Uri<String>, Value>) -> Self {
        Self { store: schemas }
    }
}

impl Retrieve for RetrieveHandler {
    fn retrieve(
        &self,
        uri: &fluent_uri::Uri<std::string::String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        match self.store.get(uri.as_str()) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Schema {uri} not found").into()),
        }
    }
}
