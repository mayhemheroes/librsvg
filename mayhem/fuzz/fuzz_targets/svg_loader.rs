#![no_main]

use glib;
use libfuzzer_sys::fuzz_target;
use rsvg;

// Preserves the mayhemheroes fork's `svg_loader` harness: exercise ONLY the SVG loader/parser
// (rsvg::Loader::read_stream) without the Cairo render step — a focused parser target. The original
// fork file referenced the crate as `librsvg`; the crate's lib name is `rsvg`, so use that.
fuzz_target!(|data: &[u8]| {
    let bytes = glib::Bytes::from(data);
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    let _ =
        rsvg::Loader::new().read_stream(&stream, None::<&gio::File>, None::<&gio::Cancellable>);
});
