use anyhow::anyhow;
use jsonschema::SchemaResolver;
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use url::Url;

#[derive(Clone, Default)]
pub struct Resolver {
    store: Arc<RwLock<HashMap<Url, Arc<Value>>>>,
}

impl Resolver {
    pub fn new(schemas: Arc<RwLock<HashMap<Url, Arc<Value>>>>) -> Self {
        Self { store: schemas }
    }
}

impl SchemaResolver for Resolver {
    fn resolve(
        &self,
        _root_schema: &Value,
        url: &Url,
        _original_reference: &str,
    ) -> Result<std::sync::Arc<Value>, jsonschema::SchemaResolverError> {
        match self.store.read().unwrap().get(url) {
            Some(v) => Ok(v.clone()),
            None => Err(anyhow!("Schema {url} not found")),
        }
    }
}
