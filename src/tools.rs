use addin1c::{CStr1C, RawAddin, Variant};

use crate::errors::{JsonSchema1CError, ParamType};
pub type ComponentResult = Result<(), JsonSchema1CError>;

pub struct Prop<T: RawAddin> {
    pub name: &'static CStr1C,
    pub name_ru: &'static CStr1C,
    pub getter: Option<fn(&mut T, &mut ParamMut) -> ComponentResult>,
    pub setter: Option<fn(&mut T, &Param) -> ComponentResult>,
}

pub enum MethodVariant<T: RawAddin> {
    Proc(fn(&mut T, &mut Params) -> ComponentResult),
    Func(fn(&mut T, &mut Params, ret_val: &mut ParamMut) -> ComponentResult),
}

pub struct Method<T: RawAddin> {
    pub name: &'static CStr1C,
    pub name_ru: &'static CStr1C,
    pub params_count: usize,
    pub method: MethodVariant<T>,
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
        }
    }
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
}

pub struct Param<'a>(&'a Variant<'a>);

impl<'a> Param<'a> {
    pub fn new(val: &'a Variant) -> Self {
        Self(val)
    }

    pub fn get_string(&self) -> Result<String, JsonSchema1CError> {
        self.0
            .get_string()
            .map_err(|_| JsonSchema1CError::PropertyConvertType(ParamType::String))
    }

    pub fn get_bool(&self) -> Result<bool, JsonSchema1CError> {
        self.0
            .get_bool()
            .map_err(|_| JsonSchema1CError::PropertyConvertType(ParamType::Bool))
    }

    pub(self) fn i_get_string(&self, num: usize) -> Result<String, JsonSchema1CError> {
        self.0
            .get_string()
            .map_err(|_| JsonSchema1CError::ConvertParamType {
                p_type: ParamType::String,
                num,
            })
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

    pub fn set_bool(&mut self, val: bool) {
        self.0.set_bool(val)
    }

    pub fn set_empty(&mut self) {
        self.0.set_empty()
    }
}

pub struct Params<'a, 'b>(&'a mut [Variant<'b>]);

impl<'a, 'b> Params<'a, 'b> {
    pub fn new(val: &'a mut [Variant<'b>]) -> Self {
        Self(val)
    }

    pub fn get_string(&self, num: usize) -> Result<String, JsonSchema1CError> {
        let Some(param) = self.0.get(num) else {
            return Err(JsonSchema1CError::ParamNotFound(num));
        };
        Param(param).i_get_string(num)
    }

    pub fn get_mut(&mut self, num: usize) -> Result<ParamMut<'_, 'b>, JsonSchema1CError> {
        let Some(param) = self.0.get_mut(num) else {
            return Err(JsonSchema1CError::ParamNotFound(num));
        };
        Ok(ParamMut(param))
    }
}
