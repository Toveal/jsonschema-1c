use addin1c::{CStr1C, RawAddin, Variant};

use crate::errors::{JsonSchema1CError, ParamType};

pub type ComponentResult = Result<(), JsonSchema1CError>;

pub struct Prop<T: RawAddin> {
    pub name: &'static CStr1C,
    pub name_ru: &'static CStr1C,
    pub getter: Option<fn(&mut T, &mut ParamMut) -> ComponentResult>,
    pub setter: Option<fn(&mut T, &Param) -> ComponentResult>,
}

impl<T: RawAddin> Prop<T> {
    pub const fn new(
        name: &'static CStr1C,
        name_ru: &'static CStr1C,
        getter: Option<fn(&mut T, &mut ParamMut) -> ComponentResult>,
        setter: Option<fn(&mut T, &Param) -> ComponentResult>,
    ) -> Self {
        Self {
            name,
            name_ru,
            getter,
            setter,
        }
    }

    pub const fn read_only(
        name: &'static CStr1C,
        name_ru: &'static CStr1C,
        getter: fn(&mut T, &mut ParamMut) -> ComponentResult,
    ) -> Self {
        Self::new(name, name_ru, Some(getter), None)
    }

    pub const fn read_write(
        name: &'static CStr1C,
        name_ru: &'static CStr1C,
        getter: fn(&mut T, &mut ParamMut) -> ComponentResult,
        setter: fn(&mut T, &Param) -> ComponentResult,
    ) -> Self {
        Self::new(name, name_ru, Some(getter), Some(setter))
    }
}

pub enum MethodVariant<T: RawAddin> {
    Proc(fn(&mut T, &mut Params) -> ComponentResult),
    Func(fn(&mut T, &mut Params, &mut ParamMut) -> ComponentResult),
}

pub struct Method<T: RawAddin> {
    pub name: &'static CStr1C,
    pub name_ru: &'static CStr1C,
    pub params_count: usize,
    pub method: MethodVariant<T>,
    pub save_error: bool,
}

impl<T: RawAddin> Method<T> {
    pub const fn new(
        name: &'static CStr1C,
        name_ru: &'static CStr1C,
        params_count: usize,
        method: MethodVariant<T>,
    ) -> Self {
        Self {
            name,
            name_ru,
            params_count,
            method,
            save_error: false,
        }
    }

    pub const fn proc(
        name: &'static CStr1C,
        name_ru: &'static CStr1C,
        params_count: usize,
        method: fn(&mut T, &mut Params) -> ComponentResult,
    ) -> Self {
        Self::new(name, name_ru, params_count, MethodVariant::Proc(method))
    }

    pub const fn func(
        name: &'static CStr1C,
        name_ru: &'static CStr1C,
        params_count: usize,
        method: fn(&mut T, &mut Params, &mut ParamMut) -> ComponentResult,
    ) -> Self {
        Self::new(name, name_ru, params_count, MethodVariant::Func(method))
    }

    pub const fn save_error(mut self) -> Self {
        self.save_error = true;
        self
    }
}

pub struct Param<'a>(&'a Variant<'a>);

impl<'a> Param<'a> {
    pub fn new(val: &'a Variant<'a>) -> Self {
        Self(val)
    }

    fn convert_err(expected: ParamType) -> JsonSchema1CError {
        JsonSchema1CError::PropertyConvert(expected)
    }

    pub fn get_string(&self) -> Result<String, JsonSchema1CError> {
        self.0
            .get_string()
            .map_err(|_| Self::convert_err(ParamType::String))
    }

    pub fn get_bool(&self) -> Result<bool, JsonSchema1CError> {
        self.0
            .get_bool()
            .map_err(|_| Self::convert_err(ParamType::Bool))
    }

    pub fn get_i32(&self) -> Result<i32, JsonSchema1CError> {
        self.0
            .get_i32()
            .map_err(|_| Self::convert_err(ParamType::I32))
    }
}

pub struct ParamMut<'a, 'b>(&'a mut Variant<'b>);

impl<'a, 'b> ParamMut<'a, 'b> {
    pub fn new(val: &'a mut Variant<'b>) -> Self {
        Self(val)
    }

    pub fn set_string(&mut self, val: impl AsRef<str>) -> Result<(), JsonSchema1CError> {
        self.0
            .set_str1c(val.as_ref())
            .map_err(|_| JsonSchema1CError::OutOfMemory)
    }

    pub fn set_bool(&mut self, val: bool) -> ComponentResult {
        self.0.set_bool(val);
        Ok(())
    }

    pub fn set_i32(&mut self, val: i32) -> ComponentResult {
        self.0.set_i32(val);
        Ok(())
    }

    pub fn set_empty(&mut self) -> ComponentResult {
        self.0.set_empty();
        Ok(())
    }
}

pub struct Params<'a, 'b>(&'a mut [Variant<'b>]);

impl<'a, 'b> Params<'a, 'b> {
    pub fn new(variants: &'a mut [Variant<'b>]) -> Self {
        Self(variants)
    }

    fn get_variant(&self, index: usize) -> Result<&Variant<'b>, JsonSchema1CError> {
        self.0
            .get(index)
            .ok_or(JsonSchema1CError::ParamNotFound(index))
    }

    fn convert_err(index: usize, expected: ParamType) -> JsonSchema1CError {
        JsonSchema1CError::ParamConvert { index, expected }
    }

    pub fn get_mut(&mut self, index: usize) -> Result<ParamMut<'_, 'b>, JsonSchema1CError> {
        self.0
            .get_mut(index)
            .map(ParamMut)
            .ok_or(JsonSchema1CError::ParamNotFound(index))
    }

    pub fn get_string(&self, index: usize) -> Result<String, JsonSchema1CError> {
        self.get_variant(index)?
            .get_string()
            .map_err(|_| Self::convert_err(index, ParamType::String))
    }

    pub fn get_bool(&self, index: usize) -> Result<bool, JsonSchema1CError> {
        self.get_variant(index)?
            .get_bool()
            .map_err(|_| Self::convert_err(index, ParamType::Bool))
    }

    pub fn get_i32(&self, index: usize) -> Result<i32, JsonSchema1CError> {
        self.get_variant(index)?
            .get_i32()
            .map_err(|_| Self::convert_err(index, ParamType::I32))
    }

    pub fn get_blob(&self, index: usize) -> Result<&[u8], JsonSchema1CError> {
        self.get_variant(index)?
            .get_blob()
            .map_err(|_| Self::convert_err(index, ParamType::Blob))
    }

    pub fn get_json_value(&self, index: usize) -> Result<serde_json::Value, JsonSchema1CError> {
        let variant = self.get_variant(index)?;

        if let Ok(s) = variant.get_string() {
            return serde_json::from_str(&s).map_err(|_| Self::convert_err(index, ParamType::Json));
        }

        if let Ok(b) = variant.get_blob() {
            return serde_json::from_slice(b)
                .map_err(|_| Self::convert_err(index, ParamType::Json));
        }

        Err(Self::convert_err(index, ParamType::StringOrBlob))
    }

    pub fn get_uri(&self, index: usize) -> Result<jsonschema::Uri<String>, JsonSchema1CError> {
        let s = self.get_string(index)?;
        jsonschema::Uri::parse(s).map_err(|_| Self::convert_err(index, ParamType::Uri))
    }
}
