use crate::errors::JsonSchema1CError;
use crate::formats::FORMATS;
use crate::retrieve_handler::RetrieveHandler;
use crate::tools::{ComponentResult, Method, MethodVariant, Param, ParamMut, Params, Prop};
use addin1c::{name, str1c, CStr1C, Connection, RawAddin, Variant};
use jsonschema::Validator;
use serde_json::Value;
use std::collections::HashMap;

const METHODS: &[Method<JsonSchema1C>] = &[
    Method::func(
        name!("GetLastError"),
        name!("ПолучитьОшибку"),
        0,
        JsonSchema1C::get_last_error,
    )
    .save_error(),
    Method::func(
        name!("IsValid"),
        name!("Действителен"),
        1,
        JsonSchema1C::check_valid,
    ),
    Method::func(
        name!("Validate"),
        name!("Проверить"),
        2,
        JsonSchema1C::validate,
    ),
    Method::proc(
        name!("AddScheme"),
        name!("ДобавитьСхему"),
        1,
        JsonSchema1C::add_scheme,
    ),
    Method::proc(
        name!("DeleteScheme"),
        name!("УдалитьСхему"),
        1,
        JsonSchema1C::delete_scheme,
    ),
    Method::proc(
        name!("DeleteAllSchemes"),
        name!("УдалитьВсеСхемы"),
        0,
        JsonSchema1C::delete_all_schemes,
    ),
    Method::proc(
        name!("SetMainScheme"),
        name!("УстановитьОсновнуюСхему"),
        1,
        JsonSchema1C::set_main_schema,
    ),
    Method::func(
        name!("GetValidationError"),
        name!("ПолучитьОшибкиВалидации"),
        0,
        JsonSchema1C::get_validation_errors,
    ),
    Method::proc(
        name!("ClearMainScheme"),
        name!("ОчиститьОсновнуюСхему"),
        0,
        JsonSchema1C::clear_main_schema,
    ),
    Method::func(
        name!("ЕстьСхема"),
        name!("HasScheme"),
        1,
        JsonSchema1C::has_scheme,
    ),
    Method::func(
        name!("ПолучитьСхемы"),
        name!("GetSchemes"),
        0,
        JsonSchema1C::get_schemes,
    ),
];

const PROPS: &[Prop<JsonSchema1C>] = &[
    Prop::read_only(name!("Schema"), name!("Схема"), JsonSchema1C::get_schema),
    Prop::read_write(
        name!("Format"),
        name!("Формат"),
        JsonSchema1C::get_format,
        JsonSchema1C::set_format,
    ),
    Prop::read_write(
        name!("UseCustomFormats"),
        name!("ИспользоватьДопФорматы"),
        JsonSchema1C::get_use_custom_formats,
        JsonSchema1C::set_use_custom_formats,
    ),
    Prop::read_only(name!("Version"), name!("Версия"), JsonSchema1C::get_version),
    Prop::read_write(
        name!("IgnoreUnknownFormats"),
        name!("ИгнорироватьНеизвестныеФорматы"),
        JsonSchema1C::get_ignore_unknown_formats,
        JsonSchema1C::set_ignore_unknown_formats,
    ),
    Prop::read_write(
        name!("CheckFormats"),
        name!("ПроверятьФорматы"),
        JsonSchema1C::get_check_formats,
        JsonSchema1C::set_check_formats,
    ),
    Prop::read_write(
        name!("Draft"),
        name!("Стандарт"),
        JsonSchema1C::get_draft,
        JsonSchema1C::set_draft,
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
    last_validation_errors: Option<String>,
    draft: Option<jsonschema::Draft>,
}

// PROPS
impl JsonSchema1C {
    fn get_schema(&mut self, val: &mut ParamMut) -> ComponentResult {
        val.set_string(self.schema.as_deref().unwrap_or_default())
    }

    fn get_format(&mut self, val: &mut ParamMut) -> ComponentResult {
        val.set_string(self.output_format.as_deref().unwrap_or_default())
    }

    fn set_format(&mut self, val: &Param) -> ComponentResult {
        self.output_format = Some(val.get_string()?);
        Ok(())
    }

    fn get_use_custom_formats(&mut self, val: &mut ParamMut) -> ComponentResult {
        val.set_bool(self.use_custom_formats)
    }

    fn set_use_custom_formats(&mut self, val: &Param) -> ComponentResult {
        self.use_custom_formats = val.get_bool()?;
        Ok(())
    }

    fn get_version(&mut self, val: &mut ParamMut) -> ComponentResult {
        val.set_str1c(str1c!(env!("CARGO_PKG_VERSION")))?;
        Ok(())
    }

    fn get_ignore_unknown_formats(&mut self, val: &mut ParamMut) -> ComponentResult {
        val.set_bool(self.ignore_unknown_formats)
    }

    fn set_ignore_unknown_formats(&mut self, val: &Param) -> ComponentResult {
        self.ignore_unknown_formats = val.get_bool()?;
        Ok(())
    }

    fn get_check_formats(&mut self, val: &mut ParamMut) -> ComponentResult {
        val.set_bool(self.check_formats)
    }

    fn set_check_formats(&mut self, val: &Param) -> ComponentResult {
        self.check_formats = val.get_bool()?;
        Ok(())
    }

    fn get_draft(&mut self, val: &mut ParamMut) -> ComponentResult {
        match self.draft.as_ref() {
            Some(d) => val.set_str1c(match d {
                jsonschema::Draft::Draft4 => str1c!("Draft4"),
                jsonschema::Draft::Draft6 => str1c!("Draft6"),
                jsonschema::Draft::Draft7 => str1c!("Draft7"),
                jsonschema::Draft::Draft201909 => str1c!("Draft201909"),
                jsonschema::Draft::Draft202012 => str1c!("Draft202012"),
                _ => str1c!("Unknown Draft"),
            }),
            None => val.set_str1c(str1c!("")),
        }
    }

    fn set_draft(&mut self, val: &Param) -> ComponentResult {
        let draft_str = val.get_str1c()?;
        let draft = if draft_str == *str1c!("4") {
            jsonschema::Draft::Draft4
        } else if draft_str == *str1c!("6") {
            jsonschema::Draft::Draft6
        } else if draft_str == *str1c!("7") {
            jsonschema::Draft::Draft7
        } else if draft_str == *str1c!("2019-09") {
            jsonschema::Draft::Draft201909
        } else if draft_str == *str1c!("2020-12") {
            jsonschema::Draft::Draft202012
        } else {
            return Err(JsonSchema1CError::UnknownDraft);
        };
        self.draft = Some(draft);
        Ok(())
    }
}

// METHODS
impl JsonSchema1C {
    fn check_valid(&mut self, params: &mut Params, ret_val: &mut ParamMut) -> ComponentResult {
        let schema = self.get_schema_self()?;
        let check_value = params.get_json_value(0)?;
        ret_val.set_bool(schema.is_valid(&check_value))
    }

    fn validate(&mut self, params: &mut Params, ret_val: &mut ParamMut) -> ComponentResult {
        let schema = self.get_schema_self()?;
        let check_value = params.get_json_value(0)?;
        let mut result = params.get_mut(1)?;

        let errors: Vec<String> = schema
            .iter_errors(&check_value)
            .map(|e| self.format_validate_error(&e))
            .collect();

        let errors_json = serde_json::to_string(&errors)?;
        self.last_validation_errors = Some(errors_json.clone());

        result.set_string(errors_json)?;
        ret_val.set_bool(errors.is_empty())
    }

    fn add_scheme(&mut self, params: &mut Params) -> ComponentResult {
        let schema_value = params.get_json_value(0)?;

        let schema_uri = schema_value
            .get("$id")
            .ok_or(JsonSchema1CError::PropertyIdNotFound)?
            .as_str()
            .ok_or(JsonSchema1CError::PropertyIdNotString)?;

        let uri = jsonschema::Uri::parse(schema_uri.to_string())
            .map_err(|_| JsonSchema1CError::InvalidUri(schema_uri.to_string()))?;

        self.schema_store.insert(uri, schema_value);
        Ok(())
    }

    fn delete_scheme(&mut self, params: &mut Params) -> ComponentResult {
        let uri = params.get_uri(0)?;
        self.schema_store.remove(&uri);
        Ok(())
    }

    fn delete_all_schemes(&mut self, _params: &mut Params) -> ComponentResult {
        self.schema_store.clear();
        Ok(())
    }

    fn get_last_error(&mut self, _params: &mut Params, ret_val: &mut ParamMut) -> ComponentResult {
        match self.last_error.as_ref() {
            Some(e) => ret_val.set_string(e.to_string()),
            None => ret_val.set_empty(),
        }
    }

    fn set_main_schema(&mut self, params: &mut Params) -> ComponentResult {
        let schema_value = params.get_json_value(0)?;

        let mut options = jsonschema::options()
            .should_ignore_unknown_formats(self.ignore_unknown_formats)
            .should_validate_formats(self.check_formats);

        if self.use_custom_formats {
            for (name, func) in FORMATS {
                options = options.with_format(name, func);
            }
        }

        if let Some(d) = self.draft {
            options = options.with_draft(d);
        }

        self.compiled_schema = Some(
            options
                .with_retriever(RetrieveHandler::new(self.schema_store.clone()))
                .build(&schema_value)?,
        );
        self.schema = Some(schema_value.to_string());
        Ok(())
    }

    fn get_validation_errors(
        &mut self,
        _params: &mut Params,
        ret_val: &mut ParamMut,
    ) -> ComponentResult {
        match self.last_validation_errors.as_deref() {
            Some(e) => ret_val.set_string(e),
            None => ret_val.set_empty(),
        }
    }

    fn clear_main_schema(&mut self, _params: &mut Params) -> ComponentResult {
        self.schema = None;
        Ok(())
    }

    fn has_scheme(&mut self, params: &mut Params, ret_val: &mut ParamMut) -> ComponentResult {
        let url = params.get_uri(0)?;
        ret_val.set_bool(self.schema_store.get(&url).is_some())
    }

    fn get_schemes(&mut self, _params: &mut Params, ret_val: &mut ParamMut) -> ComponentResult {
        let result = serde_json::to_string(&self.schema_store)?;
        ret_val.set_string(result)
    }
}

impl JsonSchema1C {
    fn get_schema_self(&self) -> Result<&Validator, JsonSchema1CError> {
        self.compiled_schema
            .as_ref()
            .ok_or(JsonSchema1CError::SchemaNotInstalled)
    }

    fn format_validate_error(&self, error: &jsonschema::ValidationError) -> String {
        match &self.output_format {
            Some(fmt) => fmt
                .replace("{path}", &error.instance_path().to_string())
                .replace("{instance}", &error.instance().to_string())
                .replace("{schema_path}", &error.schema_path().to_string())
                .replace("{error}", &error.to_string()),
            None => error.to_string(),
        }
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
            Ok(()) => true,
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
            Ok(()) => true,
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
        METHODS.get(num).map_or(0, |m| m.params_count)
    }

    fn has_ret_val(&mut self, method_num: usize) -> bool {
        METHODS
            .get(method_num)
            .is_some_and(|m| matches!(m.handler, MethodVariant::Func(_)))
    }

    fn call_as_proc(&mut self, method_num: usize, params: &mut [Variant]) -> bool {
        let Some(method) = METHODS.get(method_num) else {
            return false;
        };

        let MethodVariant::Proc(proc) = method.handler else {
            return false;
        };

        let mut props = Params::new(params);
        match proc(self, &mut props) {
            Ok(()) => {
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

        let MethodVariant::Func(proc) = method.handler else {
            return false;
        };

        let mut props = Params::new(params);
        let mut ret_val = ParamMut::new(val);
        match proc(self, &mut props, &mut ret_val) {
            Ok(()) => {
                if !method.save_error {
                    self.last_error = None;
                }
                true
            }
            Err(e) => {
                self.last_error = Some(e);
                false
            }
        }
    }
}
