use std::mem;
use std::os::raw::c_uint;
use std::rc::Rc;
use gl33::*;
use ultraviolet::Vec2;

pub struct Quad {
    gl: Rc<GlFns>,
    vao: c_uint,
    ebo: c_uint,
}

impl Quad {
    pub fn new(gl: Rc<GlFns>) -> Quad {
        let vertices = [
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0)
        ];

        let triangles = [
            [0u32, 1u32, 2u32],
            [1u32, 2u32, 3u32]
        ];

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
            gl.BufferData(GL_ARRAY_BUFFER, mem::size_of_val(&vertices) as _, vertices.as_ptr() as _, GL_STATIC_DRAW);
            gl.VertexAttribPointer(0, 2, GL_FLOAT, 0, mem::size_of::<Vec2>() as _, 0 as _);
            gl.EnableVertexAttribArray(0);

            gl.GenBuffers(1, &mut ebo);
            gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
            gl.BufferData(GL_ELEMENT_ARRAY_BUFFER, mem::size_of_val(&triangles) as _, triangles.as_ptr() as _, GL_STATIC_DRAW);
        }
        Quad { gl, vao, ebo }
    }

    pub fn render(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as _);
        }
    }
}