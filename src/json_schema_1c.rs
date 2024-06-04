use std::error::Error;
use std::sync::Arc;

use native_1c::component::IComponentBase;
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
}

impl Error for JsonSchema1CError {}

impl std::fmt::Display for JsonSchema1CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scheme compilation error")
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

        true
    }
    fn is_prop_readable(&self, _prop_num: i32) -> bool {
        true
    }

    fn is_prop_writeable(&self, _prop_num: i32) -> bool {
        true
    }

    fn get_n_methods(&self) -> i32 {
        0
    }

    fn find_method(&self, method_name: &str) -> i32 {
        match method_name {
            _ => -1,
        }
    }
    fn get_method_name(&self, method_num: i32, method_alias: i32) -> &str {
        match method_num {
            _ => unreachable!(),
        }
    }
    fn get_n_params(&self, method_num: i32) -> i32 {
        match method_num {
            _ => 0,
        }
    }
    fn get_param_def_value(
        &self,
        method_num: i32,
        param_num: i32,
        var_param_def_value: &mut Variant,
    ) -> bool {
        match method_num {
            _ => return false,
        }
        true
    }
    fn has_ret_val(&self, _method_num: i32) -> bool {
        true
    }

    fn call_as_proc(&mut self, _method_num: i32, _params: Option<&mut [Variant]>) -> bool {
        true
    }

    fn call_as_func(
        &mut self,
        method_num: i32,
        ret_vals: &mut Variant,
        params: Option<&mut [Variant]>,
    ) -> bool {
        match method_num {
            _ => return false,
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
}
