use native_1c::types::Variant;

use crate::errors::JsonSchema1CError;

pub fn unpack_first_param(params: Option<&mut [Variant]>) -> Result<&Variant, JsonSchema1CError> {
    params
        .ok_or(JsonSchema1CError::ParamUnpackError)?
        .first()
        .ok_or(JsonSchema1CError::ParamUnpackError)
}
