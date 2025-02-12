#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use crate::VboxError;
use log::debug;
use std::os::raw::c_uint;
use vbox_raw::sys_lib as raw;
use vbox_raw::sys_lib::PCVBOXCAPI;

pub fn VBoxCGlueInit() -> bool {
    debug!("VBoxCGlueInit");
    let init = unsafe { raw::VBoxCGlueInit() };
    init == 0
}
pub fn pfnGetVersion() -> Result<c_uint, VboxError> {
    debug!("VBoxCGlueInit");
    let api = g_pVBoxFuncs()?;
    let ver = unsafe { (*api).pfnGetVersion };
    match ver {
        None => Err(VboxError::get_fn_error("pfnGetVersion")),
        Some(ver) => Ok(unsafe { ver() }),
    }
}
pub fn pfnGetAPIVersion() -> Result<c_uint, VboxError> {
    debug!("pfnGetAPIVersion");
    let api = g_pVBoxFuncs()?;
    let ver = unsafe { (*api).pfnGetAPIVersion };
    match ver {
        None => Err(VboxError::get_fn_error("pfnGetAPIVersion")),
        Some(ver) => Ok(unsafe { ver() }),
    }
}

pub fn get_version() -> &'static str {
    if cfg!(is_v_6_1) {
        "v6_1"
    }else if cfg!(is_v_7_0) {
        "v7_0"
    } else {
        "v7_1"
    }
}

pub fn g_pVBoxFuncs() -> Result<PCVBOXCAPI, VboxError> {
    debug!("g_pVBoxFuncs");

    if !VBoxCGlueInit() {
        return Err(VboxError::error_init());
    }
    let api = unsafe { raw::g_pVBoxFuncs };
    Ok(api)
}
