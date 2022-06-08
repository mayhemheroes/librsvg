#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let bytes = glib::Bytes::from(data);
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    let _ =
        librsvg::Loader::new().read_stream(&stream, None::<&gio::File>, None::<&gio::Cancellable>);
});
