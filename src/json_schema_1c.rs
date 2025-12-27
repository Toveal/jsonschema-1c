use crate::errors::{JsonSchema1CError, ParamType};
use crate::formats::FORMATS;
use crate::retrieve_handler::RetrieveHandler;
use crate::tools::{ComponentResult, Method, MethodVariant, Param, ParamMut, Params, Prop};
use addin1c::{name, CStr1C, Connection, RawAddin, Variant};
use jsonschema::Validator;
use serde_json::Value;
use std::collections::HashMap;

const METHODS: &'static [Method<JsonSchema1C>] = &[
    Method::new(
        name!("GetLastError"),
        name!("ПолучитьПоследнююОшибку"),
        0,
        MethodVariant::Func(JsonSchema1C::get_last_error),
    ),
    Method::new(
        name!("IsValid"),
        name!("Действителен"),
        1,
        MethodVariant::Func(JsonSchema1C::is_valid),
    ),
    Method::new(
        name!("Validate"),
        name!("Проверить"),
        2,
        MethodVariant::Func(JsonSchema1C::validate),
    ),
    Method::new(
        name!("AddScheme"),
        name!("ДобавитьСхему"),
        1,
        MethodVariant::Proc(JsonSchema1C::add_scheme),
    ),
    Method::new(
        name!("DeleteScheme"),
        name!("УдалитьСхему"),
        1,
        MethodVariant::Proc(JsonSchema1C::delete_scheme),
    ),
    Method::new(
        name!("DeleteAllSchemes"),
        name!("УдалитьВсеСхемы"),
        0,
        MethodVariant::Proc(JsonSchema1C::delete_all_schemes),
    ),
    Method::new(
        name!("SetMainScheme"),
        name!("УстановитьОсновнуюСхему"),
        1,
        MethodVariant::Proc(JsonSchema1C::set_main_schema),
    ),
];

const PROPS: &'static [Prop<JsonSchema1C>] = &[
    Prop::new(
        name!("Schema"),
        name!("Схема"),
        Some(JsonSchema1C::get_schema),
        None,
    ),
    Prop::new(
        name!("Format"),
        name!("Формат"),
        Some(JsonSchema1C::get_format),
        Some(JsonSchema1C::set_format),
    ),
    Prop::new(
        name!("UseCustomFormats"),
        name!("ИспользоватьДопФорматы"),
        Some(JsonSchema1C::get_use_custom_formats),
        Some(JsonSchema1C::set_use_custom_formats),
    ),
    Prop::new(
        name!("Version"),
        name!("Версия"),
        Some(JsonSchema1C::get_version),
        None,
    ),
    Prop::new(
        name!("IgnoreUnknownFormats"),
        name!("ИгнорироватьНеизвестныеФорматы"),
        Some(JsonSchema1C::get_ignore_unknown_formats),
        Some(JsonSchema1C::set_ignore_unknown_formats),
    ),
    Prop::new(
        name!("CheckFormats"),
        name!("ПроверятьФорматы"),
        Some(JsonSchema1C::get_check_formats),
        Some(JsonSchema1C::set_check_formats),
    ),
];

#[derive(Default)]
pub struct JsonSchema1C {
    schema: Option<String>,
    compiled_schema: Option<Validator>,
    output_format: Option<String>,
    use_custom_formats: bool,
    last_error: Option<JsonSchema1CError>,
    schema_store: HashMap<jsonschema::Uri<String>, Value>,
    ignore_unknown_formats: bool,
    check_formats: bool,
}

// PROPS
impl JsonSchema1C {
    fn get_schema(&mut self, val: &mut ParamMut) -> ComponentResult {
        val.set_string(self.schema.as_ref().unwrap_or(&String::new()))?;
        Ok(())
    }

    fn get_format(&mut self, val: &mut ParamMut) -> ComponentResult {
        val.set_string(self.output_format.as_ref().unwrap_or(&String::new()))?;
        Ok(())
    }

    fn set_format(&mut self, val: &Param) -> ComponentResult {
        self.output_format = Some(val.get_string()?);
        Ok(())
    }

    fn get_use_custom_formats(&mut self, val: &mut ParamMut) -> ComponentResult {
        Ok(val.set_bool(self.use_custom_formats))
    }

    fn set_use_custom_formats(&mut self, val: &Param) -> ComponentResult {
        self.use_custom_formats = val.get_bool()?;
        Ok(())
    }

    fn get_version(&mut self, val: &mut ParamMut) -> ComponentResult {
        Ok(val.set_string(env!("CARGO_PKG_VERSION"))?)
    }

    fn get_ignore_unknown_formats(&mut self, val: &mut ParamMut) -> ComponentResult {
        Ok(val.set_bool(self.ignore_unknown_formats))
    }

    fn set_ignore_unknown_formats(&mut self, val: &Param) -> ComponentResult {
        self.ignore_unknown_formats = val.get_bool()?;
        Ok(())
    }

    fn get_check_formats(&mut self, val: &mut ParamMut) -> ComponentResult {
        Ok(val.set_bool(self.check_formats))
    }

    fn set_check_formats(&mut self, val: &Param) -> ComponentResult {
        self.check_formats = val.get_bool()?;
        Ok(())
    }
}

// METHODS
impl JsonSchema1C {
    fn is_valid(&mut self, params: &mut Params, ret_val: &mut ParamMut) -> ComponentResult {
        let schema = self.get_schema_self()?;
        let json = params.get_string(0)?;
        let check_value: Value = serde_json::from_str(&json)?;
        Ok(ret_val.set_bool(schema.is_valid(&check_value)))
    }

    fn validate(&mut self, params: &mut Params, ret_val: &mut ParamMut) -> ComponentResult {
        let schema = self.get_schema_self()?;
        let json = params.get_string(0)?;
        let mut result = params.get_mut(1)?;
        let check_value: Value = serde_json::from_str(&json)?;
        let err_iter = schema.iter_errors(&check_value);
        let validate_result: Vec<String> = match &self.output_format {
            Some(f) => err_iter
                .map(|e| {
                    f.replace("{path}", &e.instance_path().to_string())
                        .replace("{instance}", &e.instance().to_string())
                        .replace("{schema_path}", &e.schema_path().to_string())
                        .replace("{error}", &e.to_string())
                })
                .collect(),
            None => err_iter.map(|e| e.to_string()).collect(),
        };

        let validate_result_string = serde_json::to_string(&validate_result)?;
        result.set_string(validate_result_string)?;
        Ok(ret_val.set_bool(validate_result.is_empty()))
    }

    fn add_scheme(&mut self, params: &mut Params) -> ComponentResult {
        let json = params.get_string(0)?;
        let schema_value: Value = serde_json::from_str(&json)?;
        let schema_id = schema_value
            .get("$id")
            .ok_or(JsonSchema1CError::PropertyIdNotFound)?;
        let schema_uri = schema_id
            .as_str()
            .ok_or(JsonSchema1CError::PropertyIdNotString)?;
        let uri = jsonschema::Uri::parse(schema_uri.to_string()).map_err(|_| {
            JsonSchema1CError::ConvertParamType {
                num: 0,
                p_type: ParamType::Uri,
            }
        })?;
        self.schema_store.insert(uri, schema_value);
        Ok(())
    }

    fn delete_scheme(&mut self, params: &mut Params) -> ComponentResult {
        let input = params.get_string(0)?;
        let uri = jsonschema::Uri::parse(input.to_string()).map_err(|_| {
            JsonSchema1CError::ConvertParamType {
                num: 0,
                p_type: ParamType::Uri,
            }
        })?;
        self.schema_store.remove(&uri);
        Ok(())
    }

    fn delete_all_schemes(&mut self, _params: &mut Params) -> ComponentResult {
        self.schema_store.clear();
        Ok(())
    }

    fn get_last_error(&mut self, _params: &mut Params, ret_val: &mut ParamMut) -> ComponentResult {
        match self.last_error.as_ref() {
            Some(e) => Ok(ret_val.set_string(&e.to_string())?),
            None => Ok(ret_val.set_empty()),
        }
    }

    fn set_main_schema(&mut self, params: &mut Params) -> ComponentResult {
        let json = params.get_string(0)?;
        let schema_value: Value = serde_json::from_str(&json)?;
        let mut schema_options = jsonschema::options()
            .should_ignore_unknown_formats(self.ignore_unknown_formats)
            .should_validate_formats(self.check_formats);
        if self.use_custom_formats {
            for (name, function) in FORMATS {
                schema_options = schema_options.with_format(name, function);
            }
        }
        let schema = schema_options
            .with_retriever(RetrieveHandler::new(self.schema_store.clone()))
            .build(&schema_value)?;
        self.compiled_schema = Some(schema);
        self.schema = Some(json);
        Ok(())
    }
}

impl JsonSchema1C {
    fn get_schema_self(&self) -> Result<&Validator, JsonSchema1CError> {
        Ok(self
            .compiled_schema
            .as_ref()
            .ok_or(JsonSchema1CError::SchemeNotInstalled)?)
    }
}

impl RawAddin for JsonSchema1C {
    fn init(&mut self, _interface: &'static Connection) -> bool {
        self.use_custom_formats = true;
        self.ignore_unknown_formats = true;
        self.check_formats = true;
        true
    }

    fn get_info(&mut self) -> u16 {
        2000
    }

    fn register_extension_as(&mut self) -> &CStr1C {
        name!("JsonSchema1C")
    }

    fn get_n_props(&mut self) -> usize {
        PROPS.len()
    }

    fn find_prop(&mut self, name: &CStr1C) -> Option<usize> {
        PROPS
            .iter()
            .position(|p| p.name == name || p.name_ru == name)
    }

    fn get_prop_name(&mut self, num: usize, alias: usize) -> Option<&'static CStr1C> {
        PROPS
            .get(num)
            .map(|p| if alias == 0 { p.name } else { p.name_ru })
    }

    fn get_prop_val(&mut self, num: usize, val: &mut Variant) -> bool {
        let Some(prop) = PROPS.get(num) else {
            return false;
        };

        let Some(getter) = prop.getter else {
            return false;
        };

        self.last_error = None;

        let mut prop = ParamMut::new(val);
        match getter(self, &mut prop) {
            Ok(_) => true,
            Err(e) => {
                self.last_error = Some(e);
                false
            }
        }
    }

    fn set_prop_val(&mut self, num: usize, val: &Variant) -> bool {
        let Some(prop) = PROPS.get(num) else {
            return false;
        };

        let Some(setter) = prop.setter else {
            return false;
        };

        self.last_error = None;

        let prop = Param::new(val);
        match setter(self, &prop) {
            Ok(_) => true,
            Err(e) => {
                self.last_error = Some(e);
                false
            }
        }
    }

    fn is_prop_readable(&mut self, num: usize) -> bool {
        PROPS.get(num).is_some_and(|p| p.getter.is_some())
    }

    fn is_prop_writable(&mut self, num: usize) -> bool {
        PROPS.get(num).is_some_and(|p| p.setter.is_some())
    }

    fn get_n_methods(&mut self) -> usize {
        METHODS.len()
    }

    fn find_method(&mut self, name: &CStr1C) -> Option<usize> {
        METHODS
            .iter()
            .position(|m| m.name == name || m.name_ru == name)
    }

    fn get_method_name(&mut self, num: usize, alias: usize) -> Option<&'static CStr1C> {
        METHODS
            .get(num)
            .map(|m| if alias == 0 { m.name } else { m.name_ru })
    }

    fn get_n_params(&mut self, num: usize) -> usize {
        METHODS.get(num).map(|m| m.params_count).unwrap_or(0)
    }

    fn has_ret_val(&mut self, method_num: usize) -> bool {
        METHODS
            .get(method_num)
            .map(|m| match m.method {
                MethodVariant::Func(_) => true,
                _ => false,
            })
            .unwrap_or(false)
    }

    fn call_as_proc(&mut self, method_num: usize, params: &mut [Variant]) -> bool {
        let Some(method) = METHODS.get(method_num) else {
            return false;
        };

        let MethodVariant::Proc(proc) = method.method else {
            return false;
        };

        let mut props = Params::new(params);
        match proc(self, &mut props) {
            Ok(_) => {
                self.last_error = None;
                true
            }
            Err(e) => {
                self.last_error = Some(e);
                false
            }
        }
    }

    fn call_as_func(
        &mut self,
        method_num: usize,
        params: &mut [Variant],
        val: &mut Variant,
    ) -> bool {
        let Some(method) = METHODS.get(method_num) else {
            return false;
        };

        let MethodVariant::Func(proc) = method.method else {
            return false;
        };

        let mut props = Params::new(params);
        let mut ret_val = ParamMut::new(val);
        match proc(self, &mut props, &mut ret_val) {
            Ok(_) => {
                self.last_error = None;
                true
            }
            Err(e) => {
                self.last_error = Some(e);
                false
            }
        }
    }
}
