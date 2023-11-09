use std::rc::Rc;
use gl33::GlFns;
use ultraviolet::Vec2;
use crate::{HEIGHT, Quad, Shader, Texture, WIDTH};

pub struct Font {
    texture: Texture,
    shader: Shader,
    quad: Quad
}

impl Font {
    pub fn new(gl: Rc<GlFns>) -> Font {
        Font {
            texture: Texture::new(gl.clone(), "font.png"),
            shader: Shader::new(gl.clone(), "font", false),
            quad: Quad::new(gl)
        }
    }
    pub fn render(&self, text: &[&str]) {
        let fw = 0.025;
        let fh = fw * (WIDTH as f32) / (HEIGHT as f32);
        self.shader.set();
        self.shader.uniform2f("ssize", Vec2::new(fw, fh));
        self.shader.uniform_texture("font_texture", 0, &self.texture);
        self.shader.uniform1i("font_texture", 0);
        for i in 0..text.len() {
            for (j, ch) in text[i].chars().enumerate() {
                let x = ch as i32 % 16;
                let y = 15 - ch as i32 / 16;
                self.shader.uniform2f("spos", Vec2::new(-1.0 + fw * (j as f32), 1.0 - fh * (1.0 + i as f32)));
                self.shader.uniform2f("coords", Vec2::new(x as _, y as _));

                self.quad.render();
            }
        }
    }
}