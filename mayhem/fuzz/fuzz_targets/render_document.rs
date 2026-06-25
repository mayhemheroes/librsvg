#![no_main]

use cairo;
use glib;
use libfuzzer_sys::{Corpus, fuzz_target};
use rsvg;

// Mirrors librsvg's OSS-Fuzz `render_document` harness: load an SVG from the fuzz bytes and render
// it to a fixed-size Cairo ImageSurface. Kept at parity with upstream fuzz/fuzz_targets/.
fuzz_target!(|data: &[u8]| -> Corpus {
    let width = 96.;
    let height = 96.;

    let bytes = glib::Bytes::from(data);
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    let handle =
        rsvg::Loader::new().read_stream(&stream, None::<&gio::File>, None::<&gio::Cancellable>);
    if let Ok(handle) = handle {
        let renderer = rsvg::CairoRenderer::new(&handle);

        let surface =
            cairo::ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32)
                .unwrap();
        let cr = cairo::Context::new(&surface).unwrap();
        let _ = renderer.render_document(&cr, &cairo::Rectangle::new(0.0, 0.0, width, height));
        return Corpus::Keep;
    }

    Corpus::Reject
});
