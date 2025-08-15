#![allow(non_snake_case)]
use crate::enums::BitmapFormat;
use crate::event_detail::utility::create_ns_id_from_str;
use crate::utility::process_image_to_vec;
use log::{error, trace};
use tokio::sync::mpsc;
use vbox_raw::sys_lib::{
    nsID, IFramebuffer, IFramebufferOverlay, IFramebufferVtbl, PRBool, PRInt32, PRUint32, PRUint8,
    IFRAMEBUFFER_IID_STR,
};

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
    pub channel_id: u32,
}

impl IFramebufferImpl {
    pub fn new(pixel_format: BitmapFormat, channel_id: u32) -> *mut IFramebuffer {
        let vtbl = Box::new(get_vtbl());

        let impl_instance = Box::new(Self {
            lpVtbl: Box::into_raw(vtbl),
            data: IFramebufferData {
                pixel_format,
                ..Default::default()
            },
            channel_id,
        });

        Box::into_raw(impl_instance) as *mut IFramebuffer
    }
}
fn get_vtbl() -> IFramebufferVtbl {
    IFramebufferVtbl {
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
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            bits_per_pixel: 32,
            pixel_format: BitmapFormat::BGR,
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
            return 2159738881;
        }
        Some(this) => this,
    };

    if iid.is_null() || ppv.is_null() {
        error!("QueryInterface: null pointer detected.");
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
            return 2159738881;
        }
        Some(this) => this,
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
            return 2159738881;
        }
        Some(this) => this,
    };
    this.data.ref_count -= 1;
    trace!("Release: ref_count: {}", this.data.ref_count);
    if this.data.ref_count == 0 {
        if !(*this).lpVtbl.is_null() {
            let _ = Box::from_raw((*this).lpVtbl);
        }

        let _ = Box::from_raw(this);
        return 0;
    }
    this.data.ref_count
}

unsafe extern "C" fn get_width(this: *mut IFramebuffer, width: *mut u32) -> u32 {
    trace!("GetWidth called");
    if width.is_null() {
        error!("GetWidth");
        return 2159738881;
    }
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetWidth");
            return 2159738881;
        }
        Some(this) => this,
    };
    *width = this.data.width;
    0
}

unsafe extern "C" fn get_height(this: *mut IFramebuffer, height: *mut u32) -> u32 {
    trace!("GetHeight called");
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetWidth");
            return 2159738881;
        }
        Some(this) => this,
    };
    *height = this.data.height;
    0
}

unsafe extern "C" fn get_bits_per_pixel(this: *mut IFramebuffer, bits_per_pixel: *mut u32) -> u32 {
    trace!("GetHeight called");
    let this = match validate_this_pointer(this) {
        None => {
            error_print("get_bits_per_pixel");
            return 2159738881;
        }
        Some(this) => this,
    };
    *bits_per_pixel = this.data.bits_per_pixel;
    0
}

unsafe extern "C" fn get_bytes_per_line(this: *mut IFramebuffer, bytes_per_line: *mut u32) -> u32 {
    let this = match validate_this_pointer(this) {
        None => {
            error_print("get_bits_per_pixel");
            return 2159738881;
        }
        Some(this) => this,
    };
    *bytes_per_line = (this.data.width * this.data.bits_per_pixel) / 8;
    0
}

unsafe extern "C" fn get_pixel_format(this: *mut IFramebuffer, pixel_format: *mut u32) -> u32 {
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetPixelFormat");
            return 2159738881;
        }
        Some(this) => this,
    };
    *pixel_format = this.data.pixel_format.into();
    0
}

unsafe extern "C" fn get_height_reduction(
    this: *mut IFramebuffer,
    height_reduction: *mut u32,
) -> u32 {
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetHeightReduction");
            return 2159738881;
        }
        Some(this) => this,
    };
    *height_reduction = this.data.height_reduction;
    0
}

unsafe extern "C" fn get_overlay(
    _this: *mut IFramebuffer,
    _value: *mut *mut IFramebufferOverlay,
) -> u32 {
    log_and_fail!("GetOverlay")
}

unsafe extern "C" fn get_win_id(this: *mut IFramebuffer, win_id: *mut i64) -> u32 {
    let this = match validate_this_pointer(this) {
        None => {
            error_print("GetWinId");
            return 2159738881;
        }
        Some(this) => this,
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
            return 2159738881;
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
        return 0;
    }
    2147500035
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
            return 2159738881;
        }
        Some(this) => this,
    };
    this.data.x = x;
    this.data.y = y;
    this.data.width = width;
    this.data.height = height;
    match get_channel(this.channel_id) {
        None => {}
        Some(sender) => {
            sender
                .try_send(FramebufferEventInternal::ChangeResolution(width, height))
                .unwrap_or_else(|err| error!("NotifyUpdate: Error sending message: {:?}", err));
        }
    }
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
    trace!(
        "NotifyUpdateImage: x: {}, y: {}, width: {}, height: {}, image_size: {}",
        x,
        y,
        width,
        height,
        image_size
    );
    let this = match validate_this_pointer(this) {
        None => {
            error_print("notify_update_image");
            return 2159738881;
        }
        Some(this) => this,
    };
    if image.is_null() {
        error!("NotifyUpdateImage: image is null");
        return 2147500035;
    }
    if image_size == 0 {
        error!("NotifyUpdateImage: image_size is 0");
        return 2147500035;
    }
    if width == 0 || height == 0 {
        error!("NotifyUpdateImage: width or height is 0");
        return 2147500035;
    }
    this.data.x = x;
    this.data.y = y;
    if this.data.width == 0 {
        this.data.width = width;
    }
    if this.data.height == 0 {
        this.data.height = height;
    }
    let img = match process_image_to_vec(image_size, image) {
        Ok(img) => img,
        Err(error) => {
            error!("NotifyUpdateImage: Error processing image: {:?}", error);
            return 2147500035;
        }
    };

    send_bitmap_to_channel(&mut img.as_slice(), x, y, width, height, this.data.width, this.data.height, this.channel_id, this.data.pixel_format)
        .unwrap_or_else(|err| {
            error!("NotifyUpdateImage: Error saving image: {:?}", err);
        });
    trace!("NotifyUpdateImage: image_size: {:?}", image_size);
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
    trace!(
        "NotifyChange: screen_id: {}, x_origin: {}, y_origin: {}, width: {}, height: {}",
        screen_id,
        x_origin,
        y_origin,
        width,
        height
    );

    let this = match validate_this_pointer(this) {
        None => {
            error_print("notify_change");
            return 2159738881;
        }
        Some(this) => this,
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
    width: PRUint32,
    height: PRUint32,
    bpp: PRUint32,
    supported: *mut PRBool,
) -> u32 {
    error!("VideoModeSupported. width: {}, height: {}, bpp: {}, supported: {:?}", width, height, bpp, supported);
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
    id1.m0 == id2.m0 && id1.m1 == id2.m1 && id1.m2 == id2.m2 && id1.m3 == id2.m3
}

use crate::framebuffer::{FramebufferEventInternal, FramebufferImage, CHANNEL_REGISTRY};
use crate::VboxError;

pub fn send_bitmap_to_channel(
    bitmap: &[u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    original_width: u32,
    original_height: u32,
    channel_id: u32,
    pixel_format: BitmapFormat,
) -> Result<(), VboxError> {

    let statrt = std::time::Instant::now();
    let image = if pixel_format == BitmapFormat::BGRA {
        raw_to_bmp(bitmap, width, height)
    } else {
        raw_to_jpeg(bitmap, width, height)
    };
    error!("send_bitmap_to_channel: elapsed: {:?}", statrt.elapsed().as_secs_f64());
    match get_channel(channel_id) {
        None => {
            return Err(VboxError::new(
                0,
                "save_bitmap_as_bmp_to_channel",
                "Failed to get channel".to_string(),
                None,
            ));
        }
        Some(sender) => {
            let framebuffer_image = FramebufferImage {
                data: image,
                x,
                y,
                width,
                height,
                original_width,
                original_height,
            };
            if let Err(e) = sender.try_send(FramebufferEventInternal::FramebufferImage(
                framebuffer_image,
            )) {
                log::error!("Failed to send BMP data to channel: {:?}", e);
            }
        }
    }

    Ok(())
}

fn raw_to_bmp(bitmap: &[u8], width: u32, height: u32) ->Vec<u8> {

    let mut bmp_data = Vec::new();

    // Заголовок BMP (14 байт)
    let file_size = 14 + 40 + bitmap.len(); // Общий размер файла: заголовок + данные
    bmp_data.extend_from_slice(&[
        0x42, 0x4D, // Подпись "BM"
        (file_size & 0xFF) as u8,
        ((file_size >> 8) & 0xFF) as u8,
        ((file_size >> 16) & 0xFF) as u8,
        ((file_size >> 24) & 0xFF) as u8,
        0x00, 0x00, // Зарезервировано
        0x00, 0x00, // Зарезервировано
        0x36, 0x00, 0x00, 0x00, // Смещение к данным изображения
    ]);

    // Заголовок DIB (40 байт)
    bmp_data.extend_from_slice(&[
        0x28, 0x00, 0x00, 0x00,                  // Размер DIB-заголовка (40 байт)
        (width & 0xFF) as u8,
        ((width >> 8) & 0xFF) as u8,
        ((width >> 16) & 0xFF) as u8,
        ((width >> 24) & 0xFF) as u8,           // Ширина
        (height & 0xFF) as u8,
        ((height >> 8) & 0xFF) as u8,
        ((height >> 16) & 0xFF) as u8,
        ((height >> 24) & 0xFF) as u8,          // Высота
        0x01, 0x00,                             // Количество цветовых плоскостей (1)
        0x20, 0x00,                             // Биты на пиксель (32 bpp)
        0x00, 0x00, 0x00, 0x00,                 // Без сжатия
        (bitmap.len() & 0xFF) as u8,
        ((bitmap.len() >> 8) & 0xFF) as u8,
        ((bitmap.len() >> 16) & 0xFF) as u8,
        ((bitmap.len() >> 24) & 0xFF) as u8,   // Размер данных изображения
        0x13, 0x0B, 0x00, 0x00,                 // Горизонтальное разрешение (72 DPI)
        0x13, 0x0B, 0x00, 0x00,                 // Вертикальное разрешение (72 DPI)
        0x00, 0x00, 0x00, 0x00,                 // Число цветов в палитре (0)
        0x00, 0x00, 0x00, 0x00,                 // Важные цвета (0 - все)
    ]);

    // Переворачиваем строки (добавляем их в обратном порядке)
    let row_size = (width * 4) as usize; // 4 байта на пиксель (BGRA)
    for row in (0..height).rev() {
        let start = (row as usize) * row_size;
        let end = start + row_size;
        bmp_data.extend_from_slice(&bitmap[start..end]);
    }
    bmp_data
}

fn raw_to_jpeg(bitmap: &[u8], width: u32, height: u32) ->Vec<u8> {
    use turbojpeg::{Compressor, PixelFormat, Image};

    let pitch = (width * 4) as usize;
    let image = Image {
        pixels: bitmap,
        width: width as usize,
        height: height as usize,
        pitch,
        format: PixelFormat::BGRA,
    };

    let mut compressor = Compressor::new().unwrap();
    let jpeg_data = compressor.compress_to_vec(image).unwrap();
    jpeg_data
}

fn raw_to_jpeg1(bitmap: &[u8], width: u32, height: u32) -> Vec<u8> {
    use jpeg_encoder::{Encoder, ColorType};

    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
    for px in bitmap.chunks(4) {
        // BGRA → RGB
        rgb_data.push(px[2]); // R
        rgb_data.push(px[1]); // G
        rgb_data.push(px[0]); // B
    }

    let mut jpeg_data = Vec::new();
    let mut encoder = Encoder::new(&mut jpeg_data, 90);
    encoder.encode(&rgb_data, width as u16, height as u16, ColorType::Rgb).unwrap();

    jpeg_data
}

fn get_channel(channel_id: u32) -> Option<mpsc::Sender<FramebufferEventInternal>> {
    if let Some(registry) = CHANNEL_REGISTRY.get() {
        let registry = registry.lock().ok()?;
        registry.get(&channel_id).cloned()
    } else {
        None
    }
}
