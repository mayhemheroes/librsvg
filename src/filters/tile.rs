use crate::document::AcquiredNodes;
use crate::drawing_ctx::DrawingCtx;
use crate::element::{ElementResult, SetAttributes};
use crate::node::Node;
use crate::xml::Attributes;

use super::context::{FilterContext, FilterInput, FilterOutput, FilterResult};
use super::{FilterEffect, FilterError, FilterRender, Input, Primitive};

/// The `feTile` filter primitive.
pub struct FeTile {
    base: Primitive,
    in1: Input,
}

impl Default for FeTile {
    /// Constructs a new `Tile` with empty properties.
    #[inline]
    fn default() -> FeTile {
        FeTile {
            base: Primitive::new(),
            in1: Default::default(),
        }
    }
}

impl SetAttributes for FeTile {
    fn set_attributes(&mut self, attrs: &Attributes) -> ElementResult {
        self.in1 = self.base.parse_one_input(attrs)?;
        Ok(())
    }
}

impl FilterRender for FeTile {
    fn render(
        &self,
        _node: &Node,
        ctx: &FilterContext,
        acquired_nodes: &mut AcquiredNodes<'_>,
        draw_ctx: &mut DrawingCtx,
    ) -> Result<FilterResult, FilterError> {
        let input_1 = ctx.get_input(acquired_nodes, draw_ctx, &self.in1)?;

        // feTile doesn't consider its inputs in the filter primitive subregion calculation.
        let bounds = self.base.get_bounds(ctx)?.into_irect(ctx, draw_ctx);

        let surface = match input_1 {
            FilterInput::StandardInput(input_surface) => input_surface,
            FilterInput::PrimitiveOutput(FilterOutput {
                surface: input_surface,
                bounds: input_bounds,
            }) => {
                let tile_surface = input_surface.tile(input_bounds)?;

                ctx.source_graphic().paint_image_tiled(
                    bounds,
                    &tile_surface,
                    input_bounds.x0,
                    input_bounds.y0,
                )?
            }
        };

        Ok(FilterResult {
            name: self.base.result.clone(),
            output: FilterOutput { surface, bounds },
        })
    }
}

impl FilterEffect for FeTile {
    #[inline]
    fn is_affected_by_color_interpolation_filters(&self) -> bool {
        false
    }
}
