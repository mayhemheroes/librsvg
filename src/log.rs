//! Utilities for logging messages from the library.

use once_cell::sync::Lazy;

#[macro_export]
macro_rules! rsvg_log_session {
    (
        $session:expr,
        $($arg:tt)+
    ) => {
        if $session.log_enabled() {
            println!("{}", format_args!($($arg)+));
        }
    };
}

pub fn log_enabled() -> bool {
    static ENABLED: Lazy<bool> = Lazy::new(|| ::std::env::var_os("RSVG_LOG").is_some());

    *ENABLED
}

/// Captures the basic state of a [`cairo::Context`] for logging purposes.
///
/// A librsvg "transaction" like rendering a
/// [`crate::api::SvgHandle`], which takes a Cairo context, depends on the state of the
/// context as it was passed in by the caller.  For example, librsvg may decide to
/// operate differently depending on the context's target surface type, or its current
/// transformation matrix.  This struct captures that sort of information.
#[derive(Copy, Clone, Debug, PartialEq)]
struct CairoContextState {
    surface_type: cairo::SurfaceType,
    matrix: cairo::Matrix,
}

impl CairoContextState {
    #[cfg(test)]
    fn new(cr: &cairo::Context) -> Self {
        let surface_type = cr.target().type_();
        let matrix = cr.matrix();

        Self {
            surface_type,
            matrix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_cr_state() {
        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 10, 10).unwrap();
        let cr = cairo::Context::new(&surface).unwrap();
        let state = CairoContextState::new(&cr);

        assert_eq!(
            CairoContextState {
                surface_type: cairo::SurfaceType::Image,
                matrix: cairo::Matrix::identity(),
            },
            state,
        );

        let surface = cairo::RecordingSurface::create(cairo::Content::ColorAlpha, None).unwrap();
        let cr = cairo::Context::new(&surface).unwrap();
        cr.scale(2.0, 3.0);
        let state = CairoContextState::new(&cr);

        let mut matrix = cairo::Matrix::identity();
        matrix.scale(2.0, 3.0);

        assert_eq!(
            CairoContextState {
                surface_type: cairo::SurfaceType::Recording,
                matrix,
            },
            state,
        );
    }
}
