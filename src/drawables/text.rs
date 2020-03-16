use crate::drawables::drawable_obj::DrawableObject;
use graphics::{
    math::Vec2d,
    types::{Color, FontSize},
    Context,
};
use opengl_graphics::{GlGraphics, GlyphCache};
use std::{cell::RefCell, ops::DerefMut, rc::Rc};

pub struct DrawableText<'a> {
    pos: Vec2d,
    text: String,
    size: FontSize,
    color: Color,
    glyphs: Rc<RefCell<GlyphCache<'a>>>,
}

impl<'a> DrawableText<'a> {
    pub fn new(
        pos: Vec2d,
        text: &str,
        size: FontSize,
        color: Color,
        glyphs: Rc<RefCell<GlyphCache<'a>>>,
    ) -> Self {
        DrawableText {
            pos,
            text: String::from(text),
            size,
            color,
            glyphs,
        }
    }
}

impl<'a> DrawableObject for DrawableText<'a> {
    fn position(&self) -> Vec2d {
        return self.pos;
    }

    fn draw_object_after_traslate(&self, gl: &mut GlGraphics, ctx: Context) {
        let r = self.glyphs.clone();
        let mut g = r.borrow_mut();
        graphics::text(
            self.color,
            self.size,
            &self.text,
            g.deref_mut(),
            ctx.transform,
            gl,
        )
        .expect("No");
    }
}
