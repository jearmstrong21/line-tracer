use std::os::raw::c_uint;
use std::rc::Rc;
use gl33::*;
use crate::{HEIGHT, WIDTH};

pub struct Framebuffer {
    gl: Rc<GlFns>,
    id: c_uint,
    pub(crate) color: c_uint
}

impl Framebuffer {
    pub fn new(gl: Rc<GlFns>) -> Framebuffer {
        let mut id = 0;
        let mut color = 0;
        unsafe {
            gl.GenFramebuffers(1, &mut id);
            gl.BindFramebuffer(GL_FRAMEBUFFER, id);

            gl.GenTextures(1, &mut color);
            gl.BindTexture(GL_TEXTURE_2D, color);
            gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F.0 as _, WIDTH as _, HEIGHT as _, 0, GL_RGBA, GL_FLOAT, 0 as _);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR.0 as _);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR.0 as _);
            gl.FramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, color, 0);

            gl.BindFramebuffer(GL_FRAMEBUFFER, 0);
        }
        Framebuffer { gl, id, color }
    }
    pub fn start(&self) {
        unsafe {
            self.gl.BindFramebuffer(GL_FRAMEBUFFER, self.id);
        }
    }
    pub fn end(&self) {
        unsafe {
            self.gl.BindFramebuffer(GL_FRAMEBUFFER, 0);
        }
    }
}