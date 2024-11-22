use native_1c::types::Variant;

use crate::errors::JsonSchema1CError;

pub fn unpack_first_param(params: Option<&mut [Variant]>) -> Result<&Variant, JsonSchema1CError> {
    params
        .ok_or(JsonSchema1CError::ParamUnpackError)?
        .first()
        .ok_or(JsonSchema1CError::ParamUnpackError)
}

pub fn unpack_two_params(
    params: Option<&mut [Variant]>,
) -> Result<[&mut Variant; 2], JsonSchema1CError> {
    if let Some([p1, p2]) = params {
        Ok([p1, p2])
    } else {
        Err(JsonSchema1CError::ParamUnpackError)
    }
}
