use crate::drawables::drawable_obj::DrawableObject;
use graphics::{
    math::Vec2d,
    types::{Color, Radius, Scalar},
    Context,
};
use opengl_graphics::GlGraphics;

pub struct Border {
    pos: Vec2d,
    color: Color,
    width: Radius,
    inner_width: Scalar,
    inner_height: Scalar,
}

impl Border {
    pub fn new(
        pos: Vec2d,
        color: Color,
        width: Radius,
        inner_width: Scalar,
        inner_height: Scalar,
    ) -> Self {
        Border {
            pos,
            color,
            width,
            inner_width,
            inner_height,
        }
    }
}

impl DrawableObject for Border {
    fn position(&self) -> Vec2d {
        self.pos
    }

    fn draw_object_after_traslate(&self, gl: &mut GlGraphics, ctx: Context) {
        let border = graphics::rectangle::Rectangle::new_border(self.color, self.width);

        let rect = graphics::rectangle::rectangle_by_corners(
            0.0,
            0.0,
            self.width * 2.0 + self.inner_width,
            self.width * 2.0 + self.inner_height,
        );
        border.draw(rect, &ctx.draw_state, ctx.transform, gl);
    }
}
