use vbox_raw::sys_lib::{nsresult, VARTYPE_VT_I1};
use vbox_raw::sys_lib::{PRUint8, ULONG};
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
/// Обрабатывает данные из указателя `PRUint8` и преобразует их в `Vec<u8>`.
///
/// # Аргументы:
/// - `image_size`: Размер данных.
/// - `image`: Указатель на массив данных.
///
/// # Возврат:
/// - `Ok(Vec<u8>)`: Если данные успешно скопированы.
/// - `Err(VboxError)`: Если возникла ошибка.
pub fn process_image_to_vec(image_size: PRUint32, image: *mut PRUint8) -> Result<Vec<u8>, VboxError> {
    // Проверяем входные данные
    if image.is_null() || image_size == 0 {
        return Err(VboxError::new(
            0,
            "process_image_to_vec",
            "Null pointer or zero size".to_string(),
            None,
        ));
    }

    debug!(
        "process_image_to_vec called with image_size = {}, image pointer = {:p}",
        image_size, image
    );

    // Получаем интерфейс VirtualBox API
    let api = g_pVBoxFuncs()?;

    // 1. Создаём SAFEARRAY
    let create_safe_array = unsafe { (*api).pfnSafeArrayCreateVector }
        .ok_or_else(|| VboxError::get_fn_error("pfnSafeArrayCreateVector not found"))?;

    let safearray = unsafe { create_safe_array(VARTYPE_VT_I1, 0, image_size) };

    if safearray.is_null() {
        error!(
            "Failed to create SAFEARRAY. image_size = {}, element_size = {}",
            image_size,
            std::mem::size_of::<u8>()
        );
        return Err(VboxError::null_pointer_error("Failed to create SAFEARRAY"));
    }

    debug!("SAFEARRAY created successfully: {:p}", safearray);

    // 2. Копируем данные в SAFEARRAY
    let copy_in_fn = unsafe { (*api).pfnSafeArrayCopyInParamHelper }
        .ok_or_else(|| VboxError::get_fn_error("pfnSafeArrayCopyInParamHelper not found"))?;

    let copy_result = unsafe {
        copy_in_fn(
            safearray,
            image as *const ::std::os::raw::c_void,
            image_size as _,
        )
    };

    if copy_result != NS_OK as nsresult {
        error!("Failed to copy data into SAFEARRAY. Result: {}", copy_result);
        unsafe {
            (*api)
                .pfnSafeArrayDestroy
                .map(|destroy_fn| destroy_fn(safearray));
        }
        return Err(VboxError::get_fn_error("pfnSafeArrayCopyInParamHelper failed"));
    }

    debug!("Data successfully copied into SAFEARRAY");

    // 3. Копируем данные из SAFEARRAY в Vec<u8>
    let copy_out_fn = unsafe { (*api).pfnSafeArrayCopyOutParamHelper }
        .ok_or_else(|| VboxError::get_fn_error("pfnSafeArrayCopyOutParamHelper not found"))?;

    let mut buffer_ptr: *mut ::std::os::raw::c_void = std::ptr::null_mut();
    let mut element_count: ULONG = 0;

    let copy_out_result = unsafe {
        copy_out_fn(
            &mut buffer_ptr,
            &mut element_count,
            VARTYPE_VT_I1,
            safearray,
        )
    };

    if copy_out_result != NS_OK as nsresult {
        error!("Failed to copy data out of SAFEARRAY. Result: {}", copy_out_result);
        unsafe {
            (*api)
                .pfnSafeArrayDestroy
                .map(|destroy_fn| destroy_fn(safearray));
        }
        return Err(VboxError::get_fn_error("pfnSafeArrayCopyOutParamHelper failed"));
    }

    if buffer_ptr.is_null() || element_count == 0 {
        error!("SAFEARRAY contained null or zero elements");
        unsafe {
            (*api)
                .pfnSafeArrayDestroy
                .map(|destroy_fn| destroy_fn(safearray));
        }
        return Err(VboxError::new(
            0,
            "process_image_to_vec",
            "SAFEARRAY returned null or zero elements".to_string(),
            None,
        ));
    }

    debug!(
        "Data successfully copied out of SAFEARRAY: buffer_ptr = {:p}, element_count = {}",
        buffer_ptr, element_count
    );

    // Копируем данные из указателя в вектор
    let data = unsafe {
        std::slice::from_raw_parts(buffer_ptr as *const u8, element_count as usize).to_vec()
    };

    // Освобождаем временную память, связанную с SAFEARRAY
    let free_fn = unsafe { (*api).pfnArrayOutFree }
        .ok_or_else(|| VboxError::get_fn_error("pfnArrayOutFree not found"))?;

    unsafe { free_fn(buffer_ptr) };

    // Уничтожаем SAFEARRAY
    let destroy_fn = unsafe { (*api).pfnSafeArrayDestroy }
        .ok_or_else(|| VboxError::get_fn_error("pfnSafeArrayDestroy not found"))?;

    let destroy_result = unsafe { destroy_fn(safearray) };
    if destroy_result != NS_OK as nsresult {
        error!("Failed to destroy SAFEARRAY. Result: {}", destroy_result);
        return Err(VboxError::get_fn_error("Failed to destroy SAFEARRAY"));
    }

    debug!("SAFEARRAY destroyed successfully");

    // Возвращаем результат в виде вектора
    Ok(data)
}