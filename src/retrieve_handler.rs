use jsonschema::Retrieve;
use jsonschema::Uri;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct RetrieveHandler {
    store: HashMap<Uri<String>, Value>,
}

impl RetrieveHandler {
    pub fn new(store: HashMap<Uri<String>, Value>) -> RetrieveHandler {
        Self { store }
    }
}

impl Retrieve for RetrieveHandler {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        match self.store.get(uri.as_str()) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Schema {uri} not found").into()),
        }
    }
}
