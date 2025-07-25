use std::collections::HashMap;
use std::error::Error;

use fluent_uri::Uri;
use native_1c::component::IComponentBase;
use native_1c::native_macro::native_object;
use native_1c::types::Variant;
use serde_json::Value;

use crate::errors::JsonSchema1CError;
use crate::formats::FORMATS;
use crate::retrieve_handler::RetrieveHandler;
use crate::tools::{unpack_first_param, unpack_two_params};

type Params<'a> = Option<&'a mut [Variant]>;

#[native_object]
#[repr(C)]
pub struct JsonSchema1C {
    schema: Option<String>,
    compiled_schema: Option<jsonschema::Validator>,
    output_format: Option<String>,
    use_custom_formats: bool,
    resolver: RetrieveHandler,
    last_error: Option<Box<dyn Error>>,
    schema_store: HashMap<Uri<String>, Value>,
    ignore_unknown_formats: bool,
}

impl IComponentBase for JsonSchema1C {
    fn init(&mut self) -> bool {
        self.use_custom_formats = true;
        self.ignore_unknown_formats = true;
        true
    }

    fn get_info(&self) -> i32 {
        2000
    }

    fn done(&mut self) {}

    fn get_n_props(&self) -> i32 {
        5
    }

    fn find_prop(&self, prop_name: &str) -> i32 {
        match prop_name {
            "Schema" | "Схема" => 0,
            "Format" | "Формат" => 1,
            "UseCustomFormats" | "ИспользоватьДопФорматы" => 2,
            "Version" | "Версия" => 3,
            "IgnoreUnknownFormats" | "ИгнорироватьНеизвестныеФорматы" => {
                4
            }
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
            (4, 0) => "IgnoreUnknownFormats",
            (4, 1) => "ИгнорироватьНеизвестныеФорматы",
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
            2 => *var_prop_val = Variant::from(self.use_custom_formats),
            3 => *var_prop_val = Variant::utf16_string(self, std::env!("CARGO_PKG_VERSION")),
            4 => *var_prop_val = Variant::from(self.ignore_unknown_formats),
            _ => return false,
        }
        true
    }

    fn set_prop_val(&mut self, prop_num: i32, var_prop_val: &Variant) -> bool {
        match prop_num {
            1 => {
                if let Some(value) = var_prop_val.as_string() {
                    if value.is_empty() {
                        self.output_format = None;
                    } else {
                        self.output_format = Some(value);
                    }
                } else {
                    return false;
                };
            }
            2 => {
                if let Some(value) = var_prop_val.as_bool() {
                    self.use_custom_formats = value;
                } else {
                    return false;
                };
            }
            4 => {
                if let Some(value) = var_prop_val.as_bool() {
                    self.ignore_unknown_formats = value;
                } else {
                    return false;
                }
            }
            _ => return false,
        }

        true
    }

    fn is_prop_readable(&self, _prop_num: i32) -> bool {
        true
    }

    fn is_prop_writeable(&self, prop_num: i32) -> bool {
        !matches!(prop_num, 0 | 3)
    }

    fn get_n_methods(&self) -> i32 {
        7
    }

    fn find_method(&self, method_name: &str) -> i32 {
        match method_name {
            "IsValid" | "Действителен" => 0,
            "Validate" | "Проверить" => 1,
            "AddScheme" | "ДобавитьСхему" => 2,
            "DeleteScheme" | "УдалитьСхему" => 3,
            "DeleteAllSchemes" | "УдалитьВсеСхемы" => 4,
            "GetLastError" | "ПолучитьПоследнююОшибку" => 5,
            "SetMainScheme" | "УстановитьОсновнуюСхему" => 6,
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
            (5, 0) => "GetLastError",
            (5, 1) => "ПолучитьПоследнююОшибку",
            (6, 0) => "SetMainScheme",
            (6, 1) => "УстановитьОсновнуюСхему",
            _ => unreachable!(),
        }
    }

    fn get_n_params(&self, method_num: i32) -> i32 {
        match method_num {
            0 | 2 | 3 | 6 => 1,
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
        matches!(method_num, 0 | 1 | 5)
    }

    fn call_as_proc(&mut self, method_num: i32, params: Option<&mut [Variant]>) -> bool {
        let call_result = match method_num {
            2 => self.add_additional_scheme(params),
            3 => self.remove_additional_scheme(params),
            4 => {
                self.clear_additional_schemes();
                Ok(())
            }
            6 => self.set_main_schema(params),
            _ => unreachable!(),
        };
        match call_result {
            Ok(()) => true,
            Err(e) => {
                self.add_error(e);
                false
            }
        }
    }

    fn call_as_func(
        &mut self,
        method_num: i32,
        ret_vals: &mut Variant,
        params: Option<&mut [Variant]>,
    ) -> bool {
        let call_result = match method_num {
            0 => self.is_valid(params),
            1 => self.validate(params),
            5 => Ok(self.get_last_error()),
            _ => unreachable!(),
        };

        match call_result {
            Ok(v) => {
                *ret_vals = v;
                true
            }
            Err(e) => {
                self.add_error(e);
                false
            }
        }
    }

    fn set_locale(&mut self, _loc: &str) {}
}

impl JsonSchema1C {
    fn set_main_schema(&mut self, params: Params) -> Result<(), Box<dyn Error>> {
        let param_value = unpack_first_param(params)?
            .as_string()
            .ok_or(JsonSchema1CError::StringConversionError { n_param: 1 })?;

        let schema_value: serde_json::Value =
            serde_json::from_str(&param_value).map_err(JsonSchema1CError::from)?;

        let mut schema_options = jsonschema::options()
            .should_ignore_unknown_formats(self.ignore_unknown_formats)
            .should_validate_formats(true);

        if self.use_custom_formats {
            for (name, function) in FORMATS {
                schema_options = schema_options.with_format(name, function);
            }
        }

        let schema = schema_options
            .with_retriever(RetrieveHandler::new(self.schema_store.clone()))
            .build(&schema_value)
            .map_err(JsonSchema1CError::from)?;
        self.compiled_schema = Some(schema);
        self.schema = Some(param_value);
        Ok(())
    }

    fn add_error(&mut self, error: Box<dyn Error>) {
        self.last_error = Some(error);
    }

    fn is_valid(&self, params: Params) -> Result<Variant, Box<dyn Error>> {
        if let Some(schema) = &self.compiled_schema {
            let json = unpack_first_param(params)?
                .as_string()
                .ok_or(JsonSchema1CError::StringConversionError { n_param: 1 })?;
            let check_value: serde_json::Value =
                serde_json::from_str(&json).map_err(JsonSchema1CError::from)?;
            Ok(Variant::from(schema.is_valid(&check_value)))
        } else {
            Err(JsonSchema1CError::SchemeNotInstalled.into())
        }
    }

    fn validate(&self, params: Params) -> Result<Variant, Box<dyn Error>> {
        let Some(schema) = &self.compiled_schema else {
            return Err(JsonSchema1CError::SchemeNotInstalled.into());
        };

        let [p1, p2] = unpack_two_params(params)?;
        let json = p1
            .as_string()
            .ok_or(JsonSchema1CError::StringConversionError { n_param: 1 })?;

        let check_value: serde_json::Value =
            serde_json::from_str(&json).map_err(JsonSchema1CError::from)?;

        let err_iter = schema.iter_errors(&check_value);
        let validate_result: Vec<String> = match &self.output_format {
            Some(f) => err_iter
                .map(|e| {
                    f.replace("{path}", &e.instance_path.to_string())
                        .replace("{instance}", &e.instance.to_string())
                        .replace("{schema_path}", &e.schema_path.to_string())
                        .replace("{error}", &e.to_string())
                })
                .collect(),
            None => err_iter.map(|e| e.to_string()).collect(),
        };

        if validate_result.is_empty() {
            Ok(Variant::from(true))
        } else {
            let result_json = serde_json::to_string(&validate_result)?;
            *p2 = Variant::utf16_string(self, &result_json);
            Ok(Variant::from(false))
        }
    }

    fn add_additional_scheme(&mut self, params: Params) -> Result<(), Box<dyn Error>> {
        let json = unpack_first_param(params)?
            .as_string()
            .ok_or(JsonSchema1CError::StringConversionError { n_param: 1 })?;

        let schema_value: Value = serde_json::from_str(&json).map_err(JsonSchema1CError::from)?;
        let schema_id = schema_value
            .get("$id")
            .ok_or(JsonSchema1CError::PropertyIdNotFound)?;

        let schema_uri = schema_id
            .as_str()
            .ok_or(JsonSchema1CError::PropertyIdNotString)?;
        let uri = Uri::parse(schema_uri.to_string()).map_err(JsonSchema1CError::from)?;
        self.schema_store.insert(uri, schema_value);

        Ok(())
    }

    fn remove_additional_scheme(&mut self, params: Params) -> Result<(), Box<dyn Error>> {
        let param_value = unpack_first_param(params)?
            .as_string()
            .ok_or(JsonSchema1CError::StringConversionError { n_param: 1 })?;
        let uri = Uri::parse(param_value)?;
        self.schema_store.remove(&uri);
        Ok(())
    }

    fn clear_additional_schemes(&mut self) {
        self.schema_store.clear();
    }

    fn get_last_error(&self) -> Variant {
        if let Some(e) = self.last_error.as_ref() {
            Variant::utf16_string(self, &e.to_string())
        } else {
            Variant::empty()
        }
    }
}
