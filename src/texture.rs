use std::os::raw::c_uint;
use std::path::Path;
use std::rc::Rc;
use gl33::*;
use image::EncodableLayout;

pub struct Texture {
    pub(crate) id: c_uint,
    // gl: Rc<GlFns>
}

impl Texture {
    pub fn new(gl: Rc<GlFns>, filename: &str) -> Texture {
        let data = image::open(&Path::new(&format!("images/{}", filename))).unwrap().flipv().to_rgba8();
        let mut id = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(GL_TEXTURE_2D, id);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT.0 as _);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT.0 as _);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST.0 as _);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST.0 as _);
            gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA.0 as _,
                          data.width() as _, data.height() as _, 0,
                          GL_RGBA, GL_UNSIGNED_BYTE, &data.as_bytes()[0] as *const u8 as _);
        }
        Texture {
            id,
            // gl
        }
    }

    // pub fn
}