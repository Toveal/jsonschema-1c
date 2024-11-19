#![warn(clippy::pedantic)]

mod errors;
mod formats;
mod json_schema_1c;
mod retrieve_handler;

use native_1c::component::AppCapabilities;
use native_1c::widestring::{U16CStr, U16CString};
use native_1c::OnceCell;
use std::os::raw::c_long;

static CLASS_NAMES: OnceCell<Vec<u16>> = OnceCell::new();

#[no_mangle]
unsafe extern "C" fn GetClassObject(wide_name: *const u16, component: *mut *const u8) -> c_long {
    let Ok(name) = U16CStr::from_ptr_str(wide_name).to_string() else {
        return 0;
    };
    *component = match name.as_str() {
        "JsonSchema1C" => {
            let addin = json_schema_1c::JsonSchema1C::new();
            Box::into_raw(Box::new(addin)) as *const u8
        }

        _ => return 0,
    };
    component as c_long
}

#[no_mangle]
unsafe extern "C" fn DestroyObject(_component: *mut *const u8) -> c_long {
    0
}

#[no_mangle]
unsafe extern "C" fn GetClassNames() -> *const u16 {
    CLASS_NAMES
        .get_or_init(|| {
            U16CString::from_str("JsonSchema1C")
                .unwrap()
                .as_slice_with_nul()
                .to_vec()
        })
        .as_ptr()
}

#[no_mangle]
unsafe extern "C" fn SetPlatformCapabilities(capabilities: AppCapabilities) -> AppCapabilities {
    capabilities
}
