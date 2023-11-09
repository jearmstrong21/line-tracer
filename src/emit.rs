use std::mem;
use std::os::raw::c_uint;
use std::rc::Rc;
use gl33::*;
use ultraviolet::Vec2;

pub struct EmitGeom {
    gl: Rc<GlFns>,
    vao: c_uint
}

const N: u32 = 10000;

impl EmitGeom {
    pub fn new(gl: Rc<GlFns>) -> EmitGeom {
        let mut vertices: Vec<Vec2> = vec![];
        for i in 0..N {
            vertices.push(Vec2::new(i as f32 / N as f32, 0.0));
        }

        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
            gl.BufferData(GL_ARRAY_BUFFER, (mem::size_of::<Vec2>() * vertices.len()) as _, vertices.as_ptr() as _, GL_STATIC_DRAW);
            gl.VertexAttribPointer(0, 2, GL_FLOAT, 0, mem::size_of::<Vec2>() as _, 0 as _);
            gl.EnableVertexAttribArray(0);

        }
        EmitGeom { gl, vao }
    }
    pub fn render(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
            self.gl.DrawArrays(GL_POINTS, 0, N as _);
        }
    }
}