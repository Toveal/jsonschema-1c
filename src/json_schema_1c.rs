use std::error::Error;

use native_1c::component::{IComponentBase, IComponentInit};
use native_1c::native_macro::native_object;
use native_1c::types::Variant;

#[native_object]
#[repr(C)]
pub struct JsonSchema1C {
    schema: Option<String>,
    compiled_schema: Option<jsonschema::JSONSchema>,
    output_format: Option<String>,
}

#[derive(Debug)]
pub enum JsonSchema1CError {
    SchemaCompile,
    SchemeNotInstalled,
    StringConversionError { n_param: u32 },
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
        }
    }
}

impl IComponentBase for JsonSchema1C {
    fn init(&mut self) -> bool {
        true
    }

    fn get_info(&self) -> i32 {
        2000
    }

    fn done(&mut self) {}

    fn get_n_props(&self) -> i32 {
        2
    }

    fn find_prop(&self, prop_name: &str) -> i32 {
        match prop_name {
            "Schema" | "Схема" => 0,
            "Format" | "Формат" => 1,
            _ => -1,
        }
    }

    fn get_prop_name(&self, prop_num: i32, prop_alias: i32) -> &str {
        match (prop_num, prop_alias) {
            (0, 0) => "Schema",
            (0, 1) => "Схема",
            (1, 0) => "Format",
            (1, 1) => "Формат",
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
            _ => unreachable!(),
        }
        true
    }

    fn set_prop_val(&mut self, prop_num: i32, var_prop_val: &Variant) -> bool {
        let Some(value) = var_prop_val.as_string() else {
            return false;
        };

        match prop_num {
            0 => {
                if let Err(e) = self.set_schema(value) {
                    self.raise_an_exception(&e.to_string());
                }
            }
            1 => self.output_format = Some(value),
            _ => unreachable!(),
        }

        true
    }
    fn is_prop_readable(&self, _prop_num: i32) -> bool {
        true
    }

    fn is_prop_writeable(&self, _prop_num: i32) -> bool {
        true
    }

    fn get_n_methods(&self) -> i32 {
        2
    }

    fn find_method(&self, method_name: &str) -> i32 {
        match method_name {
            "IsValid" | "Действителен" => 0,
            "Validate" | "Проверить" => 1,
            _ => -1,
        }
    }
    fn get_method_name(&self, method_num: i32, method_alias: i32) -> &str {
        match (method_num, method_alias) {
            (0, 0) => "IsValid",
            (0, 1) => "Действителен",
            (1, 0) => "Validate",
            (1, 1) => "Проверить",
            _ => unreachable!(),
        }
    }
    fn get_n_params(&self, method_num: i32) -> i32 {
        match method_num {
            0 => 1,
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
    fn has_ret_val(&self, _method_num: i32) -> bool {
        true
    }

    fn call_as_proc(&mut self, _method_num: i32, _params: Option<&mut [Variant]>) -> bool {
        false
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
                *ret_vals = Variant::from(self.validate(&json, &mut buf));
                params_mut[1] = Variant::utf16_string(self, &buf);
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
        let schema = jsonschema::JSONSchema::compile(&schema_value)
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

    fn validate(&self, json: &String, result: &mut String) -> bool {
        todo!()
    }
}
