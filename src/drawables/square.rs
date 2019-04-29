use crate::{drawables::drawable_obj::DrawableObject, utils::BGCOLOR};
use graphics::{
    math::Vec2d,
    types::{Color, Scalar},
    Context,
};
use opengl_graphics::GlGraphics;

pub struct Square {
    pos: Vec2d,
    width: Scalar,
    color: Color,
}

impl Square {
    pub fn new(pos: Vec2d, width: Scalar, color: Color) -> Self {
        Square { pos, width, color }
    }
}

impl DrawableObject for Square {
    fn position(&self) -> Vec2d {
        return self.pos;
    }

    fn draw_object_after_traslate(&self, gl: &mut GlGraphics, ctx: Context) {
        let square = graphics::rectangle::square(0.0, 0.0, self.width);
        graphics::rectangle(BGCOLOR, square, ctx.transform, gl);
        let square = graphics::rectangle::square(1.0, 1.0, self.width - 2.0);
        graphics::rectangle(self.color, square, ctx.transform, gl);
    }
}
