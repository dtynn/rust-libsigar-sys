#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
mod ffi;

use libc::c_int;
use std::ffi::CStr;
use std::str;

const SIGAR_OK: i32 = ffi::SIGAR_OK as i32;

/// Returns an error string from given code
fn error_string(sigar: *mut ffi::sigar_t, code: c_int) -> String {
    unsafe {
        let ptr = ffi::sigar_strerror(sigar, code);
        let bytes = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8(bytes)
            .ok()
            .expect("Invalid UTF8 string")
            .to_string()
    }
}

macro_rules! ffi_wrap {
    ($fname:ident, $target:ident) => {{
        let result: Result<ffi::$target, String> = unsafe {
            let mut sigar_ptr: *mut ffi::sigar_t = std::ptr::null_mut();

            let res = ffi::sigar_open(&mut sigar_ptr);
            if res != SIGAR_OK {
                return Err(error_string(sigar_ptr, res));
            }

            let mut info: ffi::$target = Default::default();

            let res = ffi::$fname(sigar_ptr, &mut info);
            if res != SIGAR_OK {
                return Err(error_string(sigar_ptr, res));
            }

            let res = ffi::sigar_close(sigar_ptr);
            if res != SIGAR_OK {
                return Err("failed to close sigar".to_string());
            }

            Ok(info)
        };

        result
    }};
}

macro_rules! ffi_wrap_destroy {
    ($fnget:ident, $fndestroy:ident, $target:ident, $trans:ident) => {
        unsafe {
            let mut sigar_ptr: *mut ffi::sigar_t = std::ptr::null_mut();

            let res = ffi::sigar_open(&mut sigar_ptr);
            if res != SIGAR_OK {
                return Err(error_string(sigar_ptr, res));
            }

            let mut info: ffi::$target = Default::default();

            let res = ffi::$fnget(sigar_ptr, &mut info);
            if res != SIGAR_OK {
                return Err(error_string(sigar_ptr, res));
            }

            let entity = $trans(info);

            let res = ffi::$fndestroy(sigar_ptr, &mut info);
            if res != SIGAR_OK {
                return Err("failed to close sigar".to_string());
            }

            let res = ffi::sigar_close(sigar_ptr);
            if res != SIGAR_OK {
                return Err("failed to close sigar".to_string());
            }

            Ok(entity)
        }
    };
}

macro_rules! value_convert {
    ($struct:ident, $src:ident, $($field:ident), +) => {
        $struct {
            $(
            $field: $src.$field,
            )+
        }
    };
    ($struct:ident, $src:expr, $($field:ident), +) => {
        $struct {
            $(
            $field: $src.$field,
            )+
        }
    };
}

pub mod cpu;
