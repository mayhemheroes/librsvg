use gdk_pixbuf_sys::{GdkPixbuf, GdkPixbufAnimation, GdkPixbufSaveFunc};
use glib_sys::GError;
use libc::{c_char, c_int, c_uint, c_void, FILE};

//const GDK_PIXBUF_FORMAT_WRITABLE: u32 = 0b1;
const GDK_PIXBUF_FORMAT_SCALABLE: u32 = 0b10;
const GDK_PIXBUF_FORMAT_THREADSAFE: u32 = 0b100;

type GdkPixbufModuleSizeFunc = Option<extern "C" fn(*mut c_int, *mut c_int, *mut c_void)>;
type GdkPixbufModulePreparedFunc =
    Option<extern "C" fn(*mut GdkPixbuf, *mut GdkPixbufAnimation, *mut c_void)>;
type GdkPixbufModuleUpdatedFunc =
    Option<extern "C" fn(*mut GdkPixbuf, c_int, c_int, c_int, c_int, *mut c_void)>;

#[repr(C)]
struct GdkPixbufModulePattern {
    prefix: *const u8,
    mask: *const u8,
    relevance: c_int,
}

#[repr(C)]
struct GdkPixbufFormat {
    name: *const u8,
    signature: *const GdkPixbufModulePattern,
    domain: *const u8,
    description: *const u8,

    mime_types: *const *const u8,
    extensions: *const *const u8,

    flags: u32,
    disabled: c_int,
    license: *const u8,
}

struct GdkPixbufModule {
    module_name: *mut c_char,
    module_path: *mut c_char,
    module: *mut c_char,

    info: Box<GdkPixbufFormat>,
    load: extern "C" fn(*mut FILE, Option<*mut GError>) -> *mut GdkPixbuf,
    load_xpm_data: extern "C" fn(*const [u8]) -> Box<GdkPixbuf>,
    begin_load: extern "C" fn(
        GdkPixbufModuleSizeFunc,
        GdkPixbufModulePreparedFunc,
        GdkPixbufModuleUpdatedFunc,
        *mut c_void,
        Option<*mut GError>,
    ) -> *mut c_void,
    stop_load: extern "C" fn(*mut c_void, Option<*mut GError>) -> c_int,
    load_increment: extern "C" fn(*mut c_void, *const u8, c_uint, Option<*mut GError>) -> c_int,
    load_animation: extern "C" fn(*mut FILE, Option<*mut GError>) -> *mut GdkPixbufAnimation,
    save: extern "C" fn(
        *mut FILE,
        *mut GdkPixbuf,
        *mut *mut c_char,
        *mut *mut c_char,
        Option<*mut GError>,
    ) -> c_int,
    save_to_callback: extern "C" fn(
        GdkPixbufSaveFunc,
        *mut c_void,
        *mut GdkPixbuf,
        *mut *mut c_char,
        *mut *mut c_char,
        Option<*mut GError>,
    ) -> c_int,
    is_save_option_supported: extern "C" fn(*const c_char) -> c_int,

    _reserved1: *mut c_void,
    _reserved2: *mut c_void,
    _reserved3: *mut c_void,
    _reserved4: *mut c_void,
}

#[no_mangle]
extern "C" fn begin_load(
    size_func: GdkPixbufModuleSizeFunc,
    prep_func: GdkPixbufModulePreparedFunc,
    update_func: GdkPixbufModuleUpdatedFunc,
    user_data: *mut c_void,
    error: Option<*mut GError>,
) -> *mut c_void {
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn stop_load(user_data: *mut c_void, error: Option<*mut GError>) -> c_int {
    1
}

#[no_mangle]
extern "C" fn load_increment(
    user_data: *mut c_void,
    buffer: *const u8,
    size: c_uint,
    error: Option<*mut GError>,
) -> c_int {
    1
}

#[no_mangle]
extern "C" fn fill_vtable(module: &mut GdkPixbufModule) {
    module.begin_load = begin_load;
    module.stop_load = stop_load;
    module.load_increment = load_increment;
}

#[no_mangle]
extern "C" fn fill_info(info: &mut GdkPixbufFormat) {
    let signature = vec![
        GdkPixbufModulePattern {
            mask: " <svg".as_ptr(),
            prefix: "*    ".as_ptr(),
            relevance: 100,
        },
        GdkPixbufModulePattern {
            mask: " <!DOCTYPE svg".as_ptr(),
            prefix: "*             ".as_ptr(),
            relevance: 100,
        },
        GdkPixbufModulePattern {
            mask: std::ptr::null(),
            prefix: std::ptr::null(),
            relevance: 0,
        },
    ];

    let mime_types = [
        "image/svg+xml\0".as_ptr(),
        "image/svg\0".as_ptr(),
        "image/svg-xml\0".as_ptr(),
        "image/vnd.adobe.svg+xml\0".as_ptr(),
        "text/xml-svg\0".as_ptr(),
        "image/svg+xml-compressed\0".as_ptr(),
        std::ptr::null(),
    ];

    let extensions = [
        "svg\0".as_ptr(),
        "svgz\0".as_ptr(),
        "svg.gz\0".as_ptr(),
        std::ptr::null(),
    ];

    info.name = "svg\0".as_ptr();
    info.signature = signature.as_ptr();
    info.description = "Scalable Vector Graphics".as_ptr(); //TODO: Gettext this
    info.mime_types = mime_types.as_ptr();
    info.extensions = extensions.as_ptr();
    info.flags = GDK_PIXBUF_FORMAT_SCALABLE | GDK_PIXBUF_FORMAT_THREADSAFE;
    info.license = "LGPLv2".as_ptr();
}
