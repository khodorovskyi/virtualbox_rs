use crate::VboxError;
use vbox_raw::sys_lib::{nsID, IEvent};

pub(crate) fn cast_event<T>(object: *mut IEvent, iid_str: &[u8; 37]) -> Result<*mut T, VboxError> {
    let mut result: *mut std::os::raw::c_void = std::ptr::null_mut();
    let iid = create_ns_id_from_str(iid_str);
    let iid_ptr = &iid as *const nsID;
    let lp_vtbl = unsafe { (*(object)).lpVtbl };

    let get_fn_ptr =
        unsafe { (*lp_vtbl).QueryInterface }.ok_or(VboxError::get_fn_error("QueryInterface"))?;
    let result_code = unsafe { get_fn_ptr(object, iid_ptr, &mut result) };
    if result_code != 0 {
        return Err(VboxError::new(
            result_code,
            "QueryInterface",
            "".to_string(),
            None,
        ));
    }
    let new_obj: *mut T = result.cast();
    Ok(new_obj)
}

pub fn create_ns_id_from_str(id_str: &[u8; 37]) -> nsID {
    let id = std::str::from_utf8(id_str).unwrap();
    let id_vec: Vec<&str> = id.split("-").collect();
    let m0 = str_to_u32(id_vec.get(0).unwrap_or(&""));
    let m1 = str_to_u16(id_vec.get(1).unwrap_or(&""));
    let m2 = str_to_u16(id_vec.get(2).unwrap_or(&""));
    let k = format!(
        "{}{}",
        id_vec.get(3).unwrap_or(&""),
        id_vec.get(4).unwrap_or(&"")
    );
    let chunks: Vec<String> = k
        .split_whitespace()
        .flat_map(|word| {
            word.chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect();
    let chunks: Vec<u8> = chunks
        .to_vec()
        .iter()
        .map(|a| str_to_u8(a.as_str()))
        .collect();
    let (first, _) = chunks.split_at(8);
    let mut m3 = [0u8; 8];
    m3.copy_from_slice(first);
    let iid = nsID { m0, m1, m2, m3 };
    iid
}

fn str_to_u32(s: &str) -> u32 {
    u32::from_str_radix(&s, 16).unwrap_or(0)
}
fn str_to_u16(s: &str) -> u16 {
    u16::from_str_radix(&s, 16).unwrap_or(0)
}
fn str_to_u8(s: &str) -> u8 {
    u8::from_str_radix(&s, 16).unwrap_or(0)
}
