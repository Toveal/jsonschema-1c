#![warn(clippy::pedantic)]

mod errors;
mod formats;
mod json_schema_1c;
mod retrieve_handler;
mod tools;

use addin1c::{create_component, destroy_component, name, AttachType};
use std::ffi::{c_int, c_void};
use std::os::raw::c_long;
use std::sync::atomic::{AtomicI32, Ordering};

static PLATFORM_CAPABILITIES: AtomicI32 = AtomicI32::new(-1);

#[no_mangle]
unsafe extern "C" fn GetClassObject(name: *const u16, component: *mut *mut c_void) -> c_long {
    match unsafe { *name } as u8 {
        b'1' => unsafe { create_component(component, json_schema_1c::JsonSchema1C::default()) },
        _ => 0,
    }
}

#[no_mangle]
unsafe extern "C" fn DestroyObject(component: *mut *mut c_void) -> c_long {
    destroy_component(component)
}

#[no_mangle]
unsafe extern "C" fn GetClassNames() -> *const u16 {
    name!("1").as_ptr()
}

#[no_mangle]
unsafe extern "C" fn SetPlatformCapabilities(capabilities: c_int) -> c_int {
    PLATFORM_CAPABILITIES.store(capabilities, Ordering::Relaxed);
    3
}

#[no_mangle]
unsafe extern "C" fn GetAttachType() -> AttachType {
    AttachType::Any
}
