use crate::drawables::drawable_obj::DrawableObject;
use crate::utils::WIN_H;
use crate::utils::WIN_W;
use graphics::math::Vec2d;
use graphics::types::Color;
use graphics::Context;
use opengl_graphics::GlGraphics;

const OVERLAY: Color = [0.0, 0.0, 0.0, 0.8];

pub struct PauseOverlay;

impl DrawableObject for PauseOverlay {
    fn position(&self) -> Vec2d {
        return [0.0, 0.0];
    }

    fn draw_object_after_traslate(&self, gl: &mut GlGraphics, ctx: Context) {
        let overlay =
            graphics::rectangle::rectangle_by_corners(0.0, 0.0, WIN_W.into(), WIN_H.into());
        graphics::rectangle(OVERLAY, overlay, ctx.transform, gl);
    }
}
