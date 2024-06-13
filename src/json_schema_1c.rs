use std::error::Error;
use std::str::FromStr;

use native_1c::component::{IComponentBase, IComponentInit};
use native_1c::native_macro::native_object;
use native_1c::types::Variant;
use serde_json::Value;
use url::Url;

use crate::formats::FORMATS;
use crate::resolver::Resolver;

#[native_object]
#[repr(C)]
pub struct JsonSchema1C {
    schema: Option<String>,
    compiled_schema: Option<jsonschema::JSONSchema>,
    output_format: Option<String>,
    use_custom_formats: bool,
    resolver: Resolver,
}

#[derive(Debug)]
pub enum JsonSchema1CError {
    SchemaCompile,
    SchemeNotInstalled,
    StringConversionError { n_param: u32 },
    PropertyIdNotFound,
    UrlConversionError,
    PropertyIdNotString,
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
        }
    }
}

impl IComponentBase for JsonSchema1C {
    fn init(&mut self) -> bool {
        self.use_custom_formats = true;
        true
    }

    fn get_info(&self) -> i32 {
        2000
    }

    fn done(&mut self) {}

    fn get_n_props(&self) -> i32 {
        4
    }

    fn find_prop(&self, prop_name: &str) -> i32 {
        match prop_name {
            "Schema" | "Схема" => 0,
            "Format" | "Формат" => 1,
            "UseCustomFormats" | "ИспользоватьДопФорматы" => 2,
            "Version" | "Версия" => 3,
            _ => -1,
        }
    }

    fn get_prop_name(&self, prop_num: i32, prop_alias: i32) -> &str {
        match (prop_num, prop_alias) {
            (0, 0) => "Schema",
            (0, 1) => "Схема",
            (1, 0) => "Format",
            (1, 1) => "Формат",
            (2, 0) => "UseCustomFormats",
            (2, 1) => "ИспользоватьДопФорматы",
            (3, 0) => "Version",
            (3, 1) => "Версия",
            _ => unreachable!(),
        }
    }

    fn get_prop_val(&self, prop_num: i32, var_prop_val: &mut Variant) -> bool {
        match prop_num {
            0 => {
                *var_prop_val =
                    Variant::utf16_string(self, self.schema.as_deref().unwrap_or_default());
            }
            1 => {
                *var_prop_val =
                    Variant::utf16_string(self, self.output_format.as_deref().unwrap_or_default());
            }
            2 => {
                *var_prop_val = Variant::from(self.use_custom_formats);
            }
            3 => *var_prop_val = Variant::utf16_string(self, std::env!("CARGO_PKG_VERSION")),
            _ => unreachable!(),
        }
        true
    }

    fn set_prop_val(&mut self, prop_num: i32, var_prop_val: &Variant) -> bool {
        match prop_num {
            0 => {
                let Some(value) = var_prop_val.as_string() else {
                    return false;
                };
                if let Err(e) = self.set_schema(value) {
                    self.raise_an_exception(&e.to_string());
                }
            }
            1 => {
                let Some(value) = var_prop_val.as_string() else {
                    return false;
                };
                self.output_format = Some(value);
            }
            2 => {
                let Some(value) = var_prop_val.as_bool() else {
                    return false;
                };
                self.use_custom_formats = value;
            }
            _ => unreachable!(),
        }

        true
    }

    fn is_prop_readable(&self, _prop_num: i32) -> bool {
        true
    }

    fn is_prop_writeable(&self, prop_num: i32) -> bool {
        prop_num != 3
    }

    fn get_n_methods(&self) -> i32 {
        5
    }

    fn find_method(&self, method_name: &str) -> i32 {
        match method_name {
            "IsValid" | "Действителен" => 0,
            "Validate" | "Проверить" => 1,
            "AddScheme" | "ДобавитьСхему" => 2,
            "DeleteScheme" | "УдалитьСхему" => 3,
            "DeleteAllSchemes" | "УдалитьВсеСхемы" => 4,
            _ => -1,
        }
    }

    fn get_method_name(&self, method_num: i32, method_alias: i32) -> &str {
        match (method_num, method_alias) {
            (0, 0) => "IsValid",
            (0, 1) => "Действителен",
            (1, 0) => "Validate",
            (1, 1) => "Проверить",
            (2, 0) => "AddScheme",
            (2, 1) => "ДобавитьСхему",
            (3, 0) => "DeleteScheme",
            (3, 1) => "УдалитьСхему",
            (4, 0) => "DeleteAllSchemes",
            (4, 1) => "УдалитьВсеСхемы",
            _ => unreachable!(),
        }
    }

    fn get_n_params(&self, method_num: i32) -> i32 {
        match method_num {
            0 | 2 | 3 => 1,
            1 => 2,
            _ => 0,
        }
    }

    fn get_param_def_value(
        &self,
        _method_num: i32,
        _param_num: i32,
        _var_param_def_value: &mut Variant,
    ) -> bool {
        true
    }

    fn has_ret_val(&self, method_num: i32) -> bool {
        matches!(method_num, 0 | 1)
    }

    fn call_as_proc(&mut self, method_num: i32, params: Option<&mut [Variant]>) -> bool {
        match method_num {
            2 => {
                let params = params.unwrap();
                let Some(value) = params.first().unwrap().as_string() else {
                    self.raise_an_exception(
                        &JsonSchema1CError::StringConversionError { n_param: 1 }.to_string(),
                    );
                    return false;
                };
                if let Err(e) = self.add_additional_scheme(&value) {
                    self.raise_an_exception(&e.to_string());
                    return false;
                }
            }
            3 => {
                let params = params.unwrap();
                let Some(key) = params.first().unwrap().as_string() else {
                    self.raise_an_exception(
                        &JsonSchema1CError::StringConversionError { n_param: 1 }.to_string(),
                    );
                    return false;
                };

                if let Err(e) = self.remove_additional_scheme(&key) {
                    self.raise_an_exception(&e.to_string());
                    return false;
                }
            }
            4 => self.clear_additional_schemes(),
            _ => unreachable!(),
        }
        true
    }

    fn call_as_func(
        &mut self,
        method_num: i32,
        ret_vals: &mut Variant,
        params: Option<&mut [Variant]>,
    ) -> bool {
        let params_mut = params.unwrap();
        let Some(json) = params_mut.first().unwrap().as_string() else {
            self.raise_an_exception(
                &JsonSchema1CError::StringConversionError { n_param: 1 }.to_string(),
            );
            return false;
        };

        match method_num {
            0 => match self.is_valid(&json) {
                Ok(v) => *ret_vals = Variant::from(v),
                Err(e) => {
                    self.raise_an_exception(&e.to_string());
                    return false;
                }
            },
            1 => {
                let mut buf = String::new();
                match self.validate(&json, &mut buf) {
                    Ok(v) => {
                        *ret_vals = Variant::from(v);
                        params_mut[1] = Variant::utf16_string(self, &buf);
                    }
                    Err(e) => {
                        self.raise_an_exception(&e.to_string());
                        return false;
                    }
                }
            }
            _ => unreachable!(),
        }
        true
    }

    fn set_locale(&mut self, _loc: &str) {}
}

impl JsonSchema1C {
    fn set_schema(&mut self, text: String) -> Result<(), Box<dyn Error>> {
        let schema_value: serde_json::Value = serde_json::from_str(&text)?;
        let mut schema_options = jsonschema::JSONSchema::options();

        if self.use_custom_formats {
            for (name, function) in FORMATS {
                schema_options.with_format(name, function);
            }
        }

        let schema = schema_options
            .with_resolver(self.resolver.clone())
            .compile(&schema_value)
            .map_err(|_| JsonSchema1CError::SchemaCompile)?;
        self.compiled_schema = Some(schema);
        self.schema = Some(text);
        Ok(())
    }

    fn raise_an_exception(&self, text: &str) {
        self.connector()
            .add_error(1006, "JsonSchema", text, 1, self.mem_manager());
    }

    fn is_valid(&self, json: &str) -> Result<bool, Box<dyn Error>> {
        let Some(schema) = &self.compiled_schema else {
            return Err(JsonSchema1CError::SchemeNotInstalled.into());
        };
        let check_value: serde_json::Value = serde_json::from_str(json)?;
        Ok(schema.is_valid(&check_value))
    }

    fn validate(&self, json: &str, buf: &mut String) -> Result<bool, Box<dyn Error>> {
        let Some(schema) = &self.compiled_schema else {
            return Err(JsonSchema1CError::SchemeNotInstalled.into());
        };

        let check_value: serde_json::Value = serde_json::from_str(json)?;
        let validate_result = schema.validate(&check_value);

        if let Err(err_it) = validate_result {
            let errors: Vec<String> = if let Some(format) = &self.output_format {
                err_it
                    .map(|e| {
                        format
                            .replace("{path}", &e.instance_path.to_string())
                            .replace("{instance}", &e.instance.to_string())
                            .replace("{schema_path}", &e.schema_path.to_string())
                            .replace("{error}", &e.to_string())
                    })
                    .collect()
            } else {
                err_it.map(|e| e.to_string()).collect()
            };
            *buf = serde_json::to_string(&errors)?;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn add_additional_scheme(&mut self, json: &str) -> Result<(), Box<dyn Error>> {
        let schema_value: Value = serde_json::from_str(json)?;
        let schema_id = schema_value
            .get("$id")
            .ok_or(JsonSchema1CError::PropertyIdNotFound)?;
        let schema_url = schema_id
            .as_str()
            .ok_or(JsonSchema1CError::PropertyIdNotString)?;
        let url = Url::from_str(schema_url).map_err(|_| JsonSchema1CError::UrlConversionError)?;

        self.resolver.add_schema(url, schema_value);
        Ok(())
    }

    fn remove_additional_scheme(&mut self, url: &str) -> Result<(), JsonSchema1CError> {
        let url = Url::from_str(url).map_err(|_| JsonSchema1CError::UrlConversionError)?;
        self.resolver.remove_schema(&url);
        Ok(())
    }

    fn clear_additional_schemes(&mut self) {
        self.resolver.clear();
    }
}
