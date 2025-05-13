use std::slice;
use log::{error, info, trace};
use crate::event_detail::utility::create_ns_id_from_str;
use vbox_raw::sys_lib::{nsID, IFramebuffer, IFramebufferOverlay, IFramebufferVtbl, PRBool, PRInt32, PRUint32, PRUint8, IFRAMEBUFFER_IID_STR};
use crate::enums::BitmapFormat;

macro_rules! log_and_fail {
    ($name:expr) => {{
        eprintln!("Method {} called, but not implemented.", $name);
        2147500033
    }};
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IFramebufferImpl {
    #[allow(non_snake_case)]
    pub lpVtbl: *mut IFramebufferVtbl,
    pub data: IFramebufferData,
}

impl IFramebufferImpl {
    pub fn new() -> *mut IFramebuffer {
        // Создаём таблицу методов vtable
        let vtbl = Box::new(get_vtbl());

        // Собираем объект IFramebufferImpl
        let impl_instance = Box::new(Self {
            lpVtbl: Box::into_raw(vtbl), // Установите указатель на таблицу vtable
            data: Default::default(),
        });

        // Приводим объект IFramebufferImpl к *mut IFramebuffer
        Box::into_raw(impl_instance) as *mut IFramebuffer
    }

}
fn get_vtbl() -> IFramebufferVtbl {
    IFramebufferVtbl{
        QueryInterface: Some(query_interface),
        AddRef: Some(add_ref),
        Release: Some(release),
        GetWidth: Some(get_width),
        GetHeight: Some(get_height),
        GetBitsPerPixel: Some(get_bits_per_pixel),
        GetBytesPerLine: Some(get_bytes_per_line),
        GetPixelFormat: Some(get_pixel_format),
        GetHeightReduction: Some(get_height_reduction),
        GetOverlay: Some(get_overlay),
        GetWinId: Some(get_win_id),
        GetCapabilities: Some(get_capabilities),
        NotifyUpdate: Some(notify_update),
        NotifyUpdateImage: Some(notify_update_image),
        NotifyChange: Some(notify_change),
        VideoModeSupported: Some(video_mode_supported),
        GetVisibleRegion: Some(get_visible_region),
        SetVisibleRegion: Some(set_visible_region),
        ProcessVHWACommand: Some(process_vhwa_command),
        Notify3DEvent: Some(notify_3d_event),

    }
}
#[derive(Debug, Clone, Copy)]
pub struct IFramebufferData {
    pub ref_count: u32,
    pub screen_id: PRUint32,
    pub x_origin: PRUint32,
    pub y_origin: PRUint32,
    pub width: PRUint32,
    pub height: PRUint32,
    pub x: PRUint32,
    pub y: PRUint32,
    pub image_size: PRUint32,
    pub image: *mut PRUint8,
    pub bits_per_pixel: u32,
    pub pixel_format: BitmapFormat,
    height_reduction: u32,
    win_id: i64,
}

impl Default for IFramebufferData {
    fn default() -> Self {
        Self {
            ref_count: 1,
            screen_id: 0,
            x_origin: 0,
            y_origin: 0,
            width: 640,
            height: 480,
            x: 0,
            y: 0,
            image_size: 0,
            image: std::ptr::null_mut(),
            bits_per_pixel: 32,
            pixel_format: BitmapFormat::PNG,
            height_reduction: 0,
            win_id: 0,
        }
    }
}

unsafe extern "C" fn query_interface(
    this: *mut IFramebuffer,
    iid: *const nsID,
    ppv: *mut *mut ::std::os::raw::c_void,
) -> u32 {
    trace!("QueryInterface called");
    let this = match validate_this_pointer(this) {
        None => {
            error_print("AddRef");
            return 2159738881
        }
        Some(this) => {this}
    };

    if iid.is_null() || ppv.is_null() {
        eprintln!("QueryInterface: null pointer detected.");
        return 2147500035;
    }
    trace!("QueryInterface: iid: {:?}", *iid);
    
    let framebuffer_iid = create_ns_id_from_str(IFRAMEBUFFER_IID_STR);
    trace!("QueryInterface: framebuffer_iid: {:?}", framebuffer_iid);
    if is_equal_ns_id(&*iid, &framebuffer_iid) {
        trace!("QueryInterface: IID matches IFramebuffer");
        *ppv = std::ptr::null_mut();
        trace!("QueryInterface: ppv: {:?}", *ppv);
        // Если совпадает, возвращаем указатель на объект.
        this.data.ref_count += 1;
        trace!("QueryInterface: ref_count: {}", this.data.ref_count);
        *ppv = this as *mut IFramebufferImpl as *mut ::std::os::raw::c_void;
        trace!("QueryInterface: ppv: {:?}", *ppv);
        return 0; // Успешное выполнение (NS_OK)
    }
    2147500034
}

unsafe extern "C" fn add_ref(this: *mut IFramebuffer) -> u32 {
    trace!("AddRef called");
    let this = match validate_this_pointer(this) {
        None => {
            error_print("AddRef");
            return 2159738881
        }
        Some(this) => {this}
    };
    this.data.ref_count += 1;
    trace!("AddRef: ref_count: {}", this.data.ref_count);
    this.data.ref_count
}

unsafe extern "C" fn release(this: *mut IFramebuffer) -> u32 {
    trace!("Release called");
    let this = match validate_this_pointer(this) {
        None => {
            error_print("Release");
            return 2159738881
        }
        Some(this) => {this}
    };
    this.data.ref_count -= 1;
    trace!("Release: ref_count: {}", this.data.ref_count);
    if this.data.ref_count == 0 {
        // Освобождаем таблицу функций Vtbl (если требуется)
        if !(*this).lpVtbl.is_null() {
            let _ = Box::from_raw((*this).lpVtbl);
        }

        // Освобождаем сам объект
        let _ = Box::from_raw(this);
        return 0;
    }
    this.data.ref_count

}

unsafe extern "C" fn get_width(this: *mut IFramebuffer, width: *mut u32) -> u32 {
    trace!("GetWidth called");
    if width.is_null() {
        error!("GetWidth");
        return 2159738881
    }
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetWidth");
            return 2159738881
        }
        Some(this) => {this}
    };
    *width = this.data.width;
    0
}

unsafe extern "C" fn get_height(this: *mut IFramebuffer, height: *mut u32) -> u32 {
    trace!("GetHeight called");
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetWidth");
            return 2159738881
        }
        Some(this) => {this}
    };
    *height = this.data.height;
    0
}

unsafe extern "C" fn get_bits_per_pixel(this: *mut IFramebuffer, bits_per_pixel: *mut u32) -> u32 {
    trace!("GetHeight called");
    let this = match validate_this_pointer(this) {
        None => {
            error_print("get_bits_per_pixel");
            return 2159738881
        }
        Some(this) => {this}
    };
    *bits_per_pixel = this.data.bits_per_pixel;
    0
}

unsafe extern "C" fn get_bytes_per_line(this: *mut IFramebuffer, bytes_per_line: *mut u32) -> u32 {
    let this = match validate_this_pointer(this) {
        None => {
            error_print("get_bits_per_pixel");
            return 2159738881
        }
        Some(this) => {this}
    };
    *bytes_per_line = (this.data.width * this.data.bits_per_pixel) / 8;
    0

}

unsafe extern "C" fn get_pixel_format(this: *mut IFramebuffer, pixel_format: *mut u32) -> u32 {
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetPixelFormat");
            return 2159738881
        }
        Some(this) => {this}
    };
    *pixel_format = this.data.pixel_format.into();
    0
}

unsafe extern "C" fn get_height_reduction(this: *mut IFramebuffer, height_reduction: *mut u32) -> u32 {
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetHeightReduction");
            return 2159738881
        }
        Some(this) => {this}
    };
    *height_reduction = this.data.height_reduction;
    0
}

unsafe extern "C" fn get_overlay(_this: *mut IFramebuffer, _value: *mut *mut IFramebufferOverlay) -> u32 {
    log_and_fail!("GetOverlay")
}

unsafe extern "C" fn get_win_id(this: *mut IFramebuffer, win_id: *mut i64) -> u32 {
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetWinId");
            return 2159738881
        }
        Some(this) => {this}
    };
    *win_id = this.data.win_id;
    0
}

unsafe extern "C" fn get_capabilities(
    this: *mut IFramebuffer,
    size: *mut u32,
    values: *mut *mut u32,
) -> u32 {
    trace!("GetCapabilities called");
    match validate_this_pointer(this) {
        None => {
            error_print("GetCapabilities");
            return 2159738881; // NS_ERROR_FAILURE
        }
        Some(this) => this,
    };
    let capabilities = vec![1, 8];
    if !size.is_null() {
        *size = capabilities.len() as u32;
    }
    if !values.is_null() {
        let buffer = Box::into_raw(capabilities.into_boxed_slice()) as *mut u32;
        *values = buffer;
        return 0; // NS_OK
    }
    2147500035 // NS_ERROR_INVALID_POINTER
}

unsafe extern "C" fn notify_update(
    this: *mut IFramebuffer,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> u32 {
    trace!("NotifyUpdate");
    let this = match validate_this_pointer(this) {
        None => {
            error_print("NotifyUpdate");
            return 2159738881
        }
        Some(this) => {this}
    };
    this.data.x = x;
    this.data.y = y;
    this.data.width = width;
    this.data.height = height;
    0
}

unsafe extern "C" fn notify_update_image(
    this: *mut IFramebuffer,
    x: PRUint32,
    y: PRUint32,
    width: PRUint32,
    height: PRUint32,
    image_size: PRUint32,
    image: *mut PRUint8,
) -> u32 {
    trace!("NotifyUpdateImage called");
    trace!("NotifyUpdateImage: x: {}, y: {}, width: {}, height: {}, image_size: {}",
        x, y, width, height, image_size);

    let this = match validate_this_pointer(this) {
        None => {
            error_print("notify_update_image");
            return 2159738881
        }
        Some(this) => {this}
    };
    if image.is_null() {
        error!("NotifyUpdateImage: image is null");
        return 2147500035
    }
    if image_size == 0 {
        error!("NotifyUpdateImage: image_size is 0");
        return 2147500035
    }
    if width == 0 || height == 0 {
        error!("NotifyUpdateImage: width or height is 0");
        return 2147500035
    }
    this.data.x = x;
    this.data.y = y;
    this.data.width = width;
    this.data.height = height;
    this.data.image_size = (width - x) * (height - y) * this.data.bits_per_pixel / 8;
    this.data.image = image;
    trace!("NotifyUpdateImage: image_size: {:?}", this.data.image_size);
    if (image as usize) % align_of::<u8>() != 0 {
        error!("NotifyUpdateImage: Pointer `image` is not properly aligned");
        return 2147500035; // NS_ERROR_INVALID_POINTER
    }
    if image_size as usize > isize::MAX as usize {
        error!("NotifyUpdateImage: image_size exceeds isize::MAX");
        return 2147500035; // NS_ERROR_INVALID_POINTER
    }

    // let screen_data_slice =
    //     unsafe { slice::from_raw_parts(image, this.data.image_size as usize) };
    // info!("Screen data: {:?}", screen_data_slice);
    0
    
}

unsafe extern "C" fn notify_change(
    this: *mut IFramebuffer,
    screen_id: PRUint32,
    x_origin: PRUint32,
    y_origin: PRUint32,
    width: PRUint32,
    height: PRUint32,
) -> u32 {
    trace!("NotifyChange called");
    trace!("NotifyChange: screen_id: {}, x_origin: {}, y_origin: {}, width: {}, height: {}",
        screen_id, x_origin, y_origin, width, height);
   
    let this = match validate_this_pointer(this) {
        None => {
            error_print("notify_change");
            return 2159738881
        }
        Some(this) => {this}
    };
    this.data.screen_id = screen_id;
    this.data.x_origin = x_origin;
    this.data.y_origin = y_origin;
    this.data.width = width;
    this.data.height = height;
    0
}

unsafe extern "C" fn video_mode_supported(
    _this: *mut IFramebuffer,
    _width: PRUint32,
    _height: PRUint32,
    _bpp: PRUint32,
    _supported: *mut PRBool
) -> u32 {
    log_and_fail!("VideoModeSupported")
}

unsafe extern "C" fn get_visible_region(
    _this: *mut IFramebuffer,
    _rectangles: *mut PRUint8,
    _count: PRUint32,
    _count_copied: *mut PRUint32,
) -> u32 {
    log_and_fail!("GetVisibleRegion")
}

unsafe extern "C" fn set_visible_region(
    _this: *mut IFramebuffer,
    _rectangles: *mut PRUint8,
    _count: PRUint32,
) -> u32 {
    log_and_fail!("SetVisibleRegion")
}

unsafe extern "C" fn process_vhwa_command(
    _this: *mut IFramebuffer,
    _command: *mut PRUint8,
    _enm_cmd: PRInt32,
    _from_guest: PRBool,
) -> u32 {
    log_and_fail!("ProcessVHWACommand")
}

unsafe extern "C" fn notify_3d_event(
    _this: *mut IFramebuffer,
    _type_: PRUint32,
    _data_size: PRUint32,
    _data: *mut PRUint8,
) -> u32 {
    log_and_fail!("Notify3DEvent")
}

pub fn validate_this_pointer(this: *mut IFramebuffer) -> Option<&'static mut IFramebufferImpl> {
    if this.is_null() {
        log::error!("The `this` pointer is null.");
        return None;
    }
    unsafe {
        let impl_ref = &mut *(this as *mut IFramebufferImpl);
        if impl_ref.lpVtbl.is_null() {
            log::error!("Invalid IFramebufferImpl: lpVtbl is null.");
            return None;
        }
        Some(impl_ref)
    }
}

fn error_print(fn_name: &str) {
    eprintln!("Error in {}: ", fn_name);
}

pub fn is_equal_ns_id(id1: &nsID, id2: &nsID) -> bool {
    id1.m0 == id2.m0
        && id1.m1 == id2.m1
        && id1.m2 == id2.m2
        && id1.m3 == id2.m3
}
