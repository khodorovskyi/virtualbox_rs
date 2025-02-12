pub(crate) mod macros {
    // #[macro_export]
    macro_rules! stringify_params {
    ( $( $param:expr ),* ) => {{
        let mut params_str = String::new();
        $(
            params_str.push_str(&format!("{}: {:?}", stringify!($param), $param));
            params_str.push_str(", ");
        )*
        params_str.pop(); // Remove trailing comma
        params_str
    }};
}
    macro_rules! get_function_result_unit {
    ($raw:expr, $fn_name:ident $(, $params:expr)* ) => {{
        use log::{debug};
        use crate::VboxError;
        use crate::utility::macros::macros::stringify_params;
        debug!("get_function_result_unit::{}", stringify!($fn_name));
        let lp_vtbl = unsafe { (*$raw).lpVtbl };

        let get_fn_ptr = unsafe { (*lp_vtbl).$fn_name }
            .ok_or(VboxError::get_fn_error(stringify!($fn_name)))?;
        let result_code = unsafe { get_fn_ptr($raw $(, $params)*) };
        if result_code != 0 {
            debug!("result_code: {}", result_code);
            let params_str = stringify_params!($( $params ),*);
            Err(VboxError::new(
                result_code,
                stringify!($fn_name),
                format!("In params: {}", params_str).to_string(),
                None
            ))
        } else {
            Ok(())
        }
    }};
}
    // #[macro_export]
    macro_rules! call_function {
    ($raw:expr, $fn_name:ident $(, $params:expr)* ) => {{
        use log::{debug};
        use crate::VboxError;
        debug!("get_function_result_unit::{}", stringify!($fn_name));
        let lp_vtbl = unsafe { (*$raw).lpVtbl };

        let get_fn_ptr = unsafe { (*lp_vtbl).$fn_name }
            .ok_or(VboxError::get_fn_error(stringify!($fn_name)))?;
        let result_code = unsafe { get_fn_ptr($raw $(, $params)*) };
        debug!("{} result = {}", stringify!($fn_name), result_code);
        Ok(result_code as i32)
    }};
}

    // #[macro_export]
    macro_rules! get_function_result_number {
    ($raw:expr, $fn_name:ident, $result_type:ty $(, $params:expr)*) => {{
        use log::{debug};
        use crate::VboxError;
        use crate::utility::macros::macros::stringify_params;
        debug!("get_function_result_number::{}", stringify!($fn_name));
        let lp_vtbl = unsafe { (*$raw).lpVtbl };

        let get_fn_ptr = unsafe { (*lp_vtbl).$fn_name }
            .ok_or(VboxError::get_fn_error(stringify!($fn_name)))?;
        let mut result: $result_type = Default::default();
        let result_code = unsafe { get_fn_ptr($raw, $($params,)*  &mut result) };
        if result_code != 0 {
            debug!("result_code: {}", result_code);
            let params_str = stringify_params!($( $params ),*);
            Err(VboxError::new(
                result_code,
                stringify!($fn_name),
                format!("In params: {}", params_str).to_string(),
                None
            ))

        }else {
            Ok(result)
        }
    }};
}

    macro_rules! get_function_result_bool {
    ($raw:expr, $fn_name:ident $(, $params:expr)*) => {{
        use log::{debug};
        use crate::VboxError;
        use crate::utility::macros::macros::stringify_params;
        debug!("get_function_result_number::{}", stringify!($fn_name));
        let lp_vtbl = unsafe { (*$raw).lpVtbl };

        let get_fn_ptr = unsafe { (*lp_vtbl).$fn_name }
            .ok_or(VboxError::get_fn_error(stringify!($fn_name)))?;
        let mut result: i32 = Default::default();
        let result_code = unsafe { get_fn_ptr($raw, $($params,)*  &mut result) };
        if result_code != 0 {
            debug!("result_code: {}", result_code);
            let params_str = stringify_params!($( $params ),*);
            Err(VboxError::new(
                result_code,
                stringify!($fn_name),
                format!("In params: {}", params_str).to_string(),
                None
            ))
        }else {
            Ok(result == 1)
        }
    }};
}

    // #[macro_export]
    macro_rules! get_function_result_pointer {
    ($raw:expr, $fn_name:ident,$result_type:ty $(, $params:expr)* ) => {{
        use log::{debug};
        use crate::VboxError;
        use crate::utility::macros::macros::stringify_params;
        debug!("get_function_result_pointer::{}", stringify!($fn_name));
        let lp_vtbl = unsafe { (*$raw).lpVtbl };

        let get_fn_ptr = unsafe { (*lp_vtbl).$fn_name }
            .ok_or(VboxError::get_fn_error(stringify!($fn_name)))?;
        let mut result: $result_type =  std::ptr::null_mut();

        let result_code = unsafe { get_fn_ptr($raw, $($params,)* &mut result) };
        if result_code != 0 {
            debug!("result_code: {}", result_code);
            let params_str = stringify_params!($( $params ),*);
            Err(VboxError::new(
                result_code,
                stringify!($fn_name),
                format!("In params: {}", params_str).to_string(),
                None
            ))

        } else if result.is_null() {
            debug!("result.is_null. fn_name:   {}", stringify!($fn_name).to_string());
            Err(VboxError::null_pointer_error(stringify!($fn_name)))
        }else{
            Ok(result)
        }

    }};
}
    macro_rules! get_function_result_pointer_vec {
    ($raw:expr, $fn_name:ident,$result_type:ty $(, $params:expr)* ) => {{
        use log::{debug};
        use crate::VboxError;
        use crate::utility::macros::macros::stringify_params;
        debug!("get_function_result_pointer::{}", stringify!($fn_name));
        let lp_vtbl = unsafe { (*$raw).lpVtbl };

        let get_fn_ptr = unsafe { (*lp_vtbl).$fn_name }
            .ok_or(VboxError::get_fn_error(stringify!($fn_name)))?;
        let mut result: *mut $result_type =  std::ptr::null_mut();
        let mut count = 0;

        let result_code = unsafe { get_fn_ptr($raw, $($params,)* &mut count, &mut result) };
        if result_code != 0 {
            debug!("result_code: {}", result_code);
            let params_str = stringify_params!($( $params ),*);
            Err(VboxError::new(
                result_code,
                stringify!($fn_name),
                format!("In params: {}", params_str).to_string(),
                None
            ))

        } else if result.is_null() {
            debug!("result.is_null. fn_name:   {}", stringify!($fn_name).to_string());
            Err(VboxError::null_pointer_error(stringify!($fn_name)))
        }else{
            let data = unsafe { Vec::from_raw_parts(result, count as usize, count as usize) };
            Ok(data)
        }

    }};
}

    macro_rules! get_function_result_str {
    ($raw:expr, $fn_name:ident $(, $params:expr)* ) => {{
            use log::{debug};
            use crate::VboxError;
            use crate::utility::macros::macros::stringify_params;
            use crate::utility::c_u64_str_to_string;

            debug!("get_function_result_pointer::{}", stringify!($fn_name));
            let lp_vtbl = unsafe { (*$raw).lpVtbl };

            let get_fn_ptr = unsafe { (*lp_vtbl).$fn_name }
                .ok_or(VboxError::get_fn_error(stringify!($fn_name)))?;
            let mut result: *mut u16 =  std::ptr::null_mut();

            let result_code = unsafe { get_fn_ptr($raw, $($params,)* &mut result) };
            if result_code != 0 {
                debug!("result_code: {}", result_code);
                let params_str = stringify_params!($( $params ),*);
                Err(VboxError::new(
                    result_code,
                    stringify!($fn_name),
                    format!("In params: {}", params_str).to_string(),
                    None
                ))

            } else if result.is_null() {
                debug!("result.is_null. fn_name:   {}", stringify!($fn_name).to_string());
                Err(VboxError::null_pointer_error(stringify!($fn_name)))
            }else{
                c_u64_str_to_string(result)
            }

        }};
    }
    macro_rules! get_function_result_str_vec {
    ($raw:expr, $fn_name:ident $(, $params:expr)* ) => {{
        use log::{debug};
        use crate::VboxError;
        use crate::utility::macros::macros::stringify_params;
        use crate::utility::c_u64_str_to_string;

        debug!("get_function_result_str_vec::{}", stringify!($fn_name));
        let lp_vtbl = unsafe { (*$raw).lpVtbl };

        let get_fn_ptr = unsafe { (*lp_vtbl).$fn_name }
            .ok_or(VboxError::get_fn_error(stringify!($fn_name)))?;
        let mut result: *mut *mut  u16 =  std::ptr::null_mut();
        let mut count = 0;
        let result_code = unsafe { get_fn_ptr($raw, $($params,)* &mut count, &mut result) };
        if result_code != 0 {
            debug!("result_code: {}", result_code);
            let params_str = stringify_params!($( $params ),*);
            Err(VboxError::new(
                result_code,
                stringify!($fn_name),
                format!("In params: {}", params_str).to_string(),
                None
            ))

        } else if result.is_null() {
            debug!("result.is_null. fn_name:   {}", stringify!($fn_name).to_string());
            Err(VboxError::null_pointer_error(stringify!($fn_name)))
        }else{
            let data = unsafe { Vec::from_raw_parts(result, count as usize, count as usize) };
            let data_str = data
                .iter()
                .map(|s| c_u64_str_to_string(s.clone()))
                .collect();
            data_str
        }

    }};
}
    pub(crate) use call_function;
    pub(crate) use get_function_result_bool;
    pub(crate) use get_function_result_number;
    pub(crate) use get_function_result_pointer;
    pub(crate) use get_function_result_pointer_vec;
    pub(crate) use get_function_result_str;
    pub(crate) use get_function_result_str_vec;
    pub(crate) use get_function_result_unit;
    pub(crate) use stringify_params;
}
