use crate::core::g_pVBoxFuncs;
use crate::{VboxError, VboxErrorType};
use log::{debug, error};
use std::ffi::{CStr, CString};
use vbox_raw::sys_lib::{PRUint32, NS_OK, VARTYPE_VT_UI2};

pub mod macros;

pub(crate) fn c_i8_str_to_string(c_str: *mut i8) -> &'static str {
    debug!("c_i8_str_to_string");
    if c_str.is_null() {
        return "";
    }

    let cstr = unsafe { CStr::from_ptr(c_str) };
    let r_str: &str = match cstr.to_str() {
        Ok(cstr) => cstr,
        Err(err) => {
            error!("c_i8_str_to_string Error. Error: {}", err);
            return "";
        }
    };
    r_str
}

pub(crate) fn c_u64_str_to_string(c_str_16: *const u16) -> Result<&'static str, VboxError> {
    debug!("c_u64_str_to_string");
    let api = g_pVBoxFuncs()?;
    let f = unsafe { (*api).pfnUtf16ToUtf8 }.ok_or(VboxError::get_fn_error("pfnUtf16ToUtf8"))?;
    let mut c_str_8: *mut i8 = std::ptr::null_mut();

    let result = unsafe { f(c_str_16, &mut c_str_8) };

    if result != NS_OK as i32 {
        return Err(VboxError::get_fn_error("c_u64_str_to_string"));
    }
    if c_str_8.is_null() {
        return Err(VboxError::get_fn_error(
            "c_u64_str_to_string Error. c_str_8.is_null()",
        ));
    }
    Ok(c_i8_str_to_string(c_str_8))
}

pub(crate) fn string_to_c_u64_str(str_utf_8: &str) -> Result<*mut u16, VboxError> {
    debug!("string_c_u64_str");
    let api = g_pVBoxFuncs()?;
    let fn_ptr =
        unsafe { (*api).pfnUtf8ToUtf16 }.ok_or(VboxError::get_fn_error("pfnUtf8ToUtf16"))?;
    let mut c_str_16: *mut u16 = std::ptr::null_mut();
    // let c_str_utf_8 =  unsafe {CStr::from_bytes_with_nul_unchecked(str_utf_8.as_bytes())};
    let c_str_utf_8 = CString::new(str_utf_8).map_err(|err| {
        VboxError::new(
            0,
            "string_to_c_u64_str",
            format!("String: {}, Error: {}", str_utf_8, err),
            Some(VboxErrorType::ConvertStringError),
        )
    })?;
    let result = unsafe { fn_ptr(c_str_utf_8.as_ptr(), &mut c_str_16) };

    if result != NS_OK as i32 {
        return Err(VboxError::get_fn_error("c_u64_str_to_string"));
    }
    if c_str_16.is_null() {
        return Err(VboxError::get_fn_error(
            "c_u64_str_to_string Error. c_str_8.is_null()",
        ));
    }
    Ok(c_str_16)
}

pub(crate) fn str_vec_to_ptr(strs: Vec<&str>) -> Result<(u32, *mut *mut u16), VboxError> {
    let mut strs_ptr_vec = Vec::new();
    for s in &strs {
        strs_ptr_vec.push(string_to_c_u64_str(s)?);
    }

    let strs_size = strs.len() as PRUint32;

    let api = g_pVBoxFuncs()?;
    let create_fn = unsafe { (*api).pfnSafeArrayCreateVector }
        .ok_or(VboxError::get_fn_error("pfnUtf8ToUtf16"))?;

    let safearray = unsafe {
        create_fn(VARTYPE_VT_UI2 /* VT_UI2 */, 0, strs_size)
    };
    if safearray.is_null() {
        return Err(VboxError::null_pointer_error("str_vec_to_ptr"));
    }

    for (i, &utf16_str) in strs_ptr_vec.iter().enumerate() {
        unsafe {
            let element_ptr = (safearray as *mut *mut u16).add(i);
            *element_ptr = utf16_str;
        }
    }
    Ok((strs_size, safearray as *mut *mut u16))
}
