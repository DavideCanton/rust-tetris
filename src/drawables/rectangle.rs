use crate::{drawables::drawable_obj::DrawableObject};
use graphics::{
    math::Vec2d,
    types::{Color, Scalar},
    Context,
};
use opengl_graphics::GlGraphics;

pub struct Rectangle {
    pos: Vec2d,
    width: Scalar,
    height: Scalar,
    color: Color,
}

impl Rectangle {
    pub fn new(pos: Vec2d, width: Scalar, height: Scalar, color: Color) -> Self {
        Rectangle {
            pos,
            width,
            height,
            color,
        }
    }
}

impl DrawableObject for Rectangle {
    fn position(&self) -> Vec2d {
        self.pos
    }

    fn draw_object_after_traslate(&self, gl: &mut GlGraphics, ctx: Context) {
        let rect = graphics::rectangle::rectangle_by_corners(0.0, 0.0, self.width, self.height);
        graphics::rectangle(self.color, rect, ctx.transform, gl);
    }
}
