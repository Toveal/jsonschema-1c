use anyhow::anyhow;
use jsonschema::SchemaResolver;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use url::Url;

#[derive(Clone, Default)]
pub struct Resolver {
    schemas: HashMap<Url, Value>,
}

impl Resolver {
    pub fn add_schema(&mut self, url: Url, schema: Value) {
        self.schemas.insert(url, schema);
    }

    pub fn remove_schema(&mut self, url: &Url) {
        self.schemas.remove(url);
    }

    pub fn clear(&mut self) {
        self.schemas.clear();
    }
}

impl SchemaResolver for Resolver {
    fn resolve(
        &self,
        _root_schema: &Value,
        url: &Url,
        _original_reference: &str,
    ) -> Result<std::sync::Arc<Value>, jsonschema::SchemaResolverError> {
        match self.schemas.get(url) {
            Some(v) => Ok(Arc::new(v.to_owned())),
            None => Err(anyhow!("Schema {url} not found")),
        }
    }
}
