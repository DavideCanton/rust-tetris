use graphics::{math::Vec2d, Context, Transformed};
use opengl_graphics::GlGraphics;

pub trait DrawableObject {
    fn position(&self) -> Vec2d;
    fn draw_object_after_traslate(&self, gl: &mut GlGraphics, ctx: Context);

    fn draw_object(&self, gl: &mut GlGraphics, ctx: Context) {
        let pos = self.position();

        let ctx = ctx.trans(pos[0], pos[1]);
        self.draw_object_after_traslate(gl, ctx);
    }
}
