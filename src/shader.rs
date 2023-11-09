use std::fs;
use std::os::raw::c_uint;
use std::path::Path;
use std::rc::Rc;
use gl33::*;
use ultraviolet::Vec2;
use crate::{Framebuffer, Texture};

fn raw_shader_source(name: &str, ext: &'static str) -> String {
    format!("{}\n", fs::read_to_string(format!("shaders/{}.{}", name, ext)).unwrap())
}
fn shader_source(name: &str, ext: &'static str, already_included: &mut Vec<String>) -> String {
    if already_included.contains(&name.to_string()) {
        return "".to_string()
    }
    already_included.push(name.to_string());
    let mut raw = raw_shader_source(name, ext);
    let mut cur = "".to_string();
    while !raw.is_empty() {
        if raw.starts_with("#include") {
            raw = (&raw[9..]).to_string();
            let mut include = "".to_string();
            while !raw.starts_with("\n") {
                include = format!("{}{}", include, raw.chars().nth(0).unwrap());
                raw = (&raw[1..]).to_string();
            }
            raw = (&raw[1..]).to_string(); // \n
            cur += &shader_source(&include, "glsl", already_included);
        } else {
            cur = format!("{}{}", cur, raw.chars().nth(0).unwrap());
            raw = (&raw[1..]).to_string();
        }
    }
    return cur
}
unsafe fn compile_shader(gl: Rc<GlFns>, name: &str, geom: bool) -> c_uint {
    let shader = gl.CreateProgram();

    let vertex = gl.CreateShader(GL_VERTEX_SHADER);
    let source = shader_source(name, "vert", &mut vec![]);
    gl.ShaderSource(vertex, 1, &(source.as_bytes().as_ptr() as _), &(source.len() as _));
    gl.CompileShader(vertex);
    let mut success = 0;
    gl.GetShaderiv(vertex, GL_COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut len = 0;
        gl.GetShaderInfoLog(vertex, 1024, &mut len, v.as_mut_ptr() as _);
        v.set_len(len as usize);
        panic!("Vertex compile error: {}\n{}", name, String::from_utf8_lossy(&v))
    }
    gl.AttachShader(shader, vertex);

    let fragment = gl.CreateShader(GL_FRAGMENT_SHADER);
    let source = shader_source(name, "frag", &mut vec![]);
    gl.ShaderSource(fragment, 1, &(source.as_bytes().as_ptr() as _), &(source.len() as _));
    gl.CompileShader(fragment);
    let mut success = 0;
    gl.GetShaderiv(fragment, GL_COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut len = 0;
        gl.GetShaderInfoLog(fragment, 1024, &mut len, v.as_mut_ptr() as _);
        v.set_len(len as usize);
        panic!("Fragment compile error: {}\n{}", name, String::from_utf8_lossy(&v))
    }
    gl.AttachShader(shader, fragment);

    if geom {
        let geometry = gl.CreateShader(GL_GEOMETRY_SHADER);
        let source = shader_source(name, "geom", &mut vec![]);
        gl.ShaderSource(geometry, 1, &(source.as_bytes().as_ptr() as _), &(source.len() as _));
        gl.CompileShader(geometry);
        let mut success = 0;
        gl.GetShaderiv(geometry, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut len = 0;
            gl.GetShaderInfoLog(geometry, 1024, &mut len, v.as_mut_ptr() as _);
            v.set_len(len as usize);
            // fs::write("error.geom", source).unwrap();
            panic!("Geometry compile error: {}\n{}", name, String::from_utf8_lossy(&v))
        }
        gl.AttachShader(shader, geometry);
    }

    gl.LinkProgram(shader);
    let mut success = 0;
    gl.GetProgramiv(shader, GL_LINK_STATUS, &mut success);
    if success == 0 {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut len = 0;
        gl.GetProgramInfoLog(shader, 1024, &mut len, v.as_mut_ptr() as _);
        v.set_len(len as usize);
        panic!("Shader link error: {}\n{}", name, String::from_utf8_lossy(&v))
    }

    shader
}

pub struct Shader {
    id: c_uint,
    gl: Rc<GlFns>
}

impl Shader {
    pub fn new(gl: Rc<GlFns>, name: &str, geom: bool) -> Shader {
        Shader {
            id: unsafe { compile_shader(gl.clone(), name, geom) },
            gl
        }
    }
    unsafe fn location(&self, name: &str) -> i32 {
        let x = std::ffi::CString::new(name).unwrap();
        self.gl.GetUniformLocation(self.id, x.as_ptr() as _)
    }
    pub fn set(&self) {
        self.gl.UseProgram(self.id)
    }
    pub fn uniform1f(&self, name: &str, value: f32) {
        unsafe { self.gl.Uniform1f(self.location(name), value) }
    }
    pub fn uniform2f(&self, name: &str, value: Vec2) {
        unsafe { self.gl.Uniform2f(self.location(name), value.x, value.y) }
    }
    pub fn uniform1i(&self, name: &str, value: i32) {
        unsafe { self.gl.Uniform1i(self.location(name), value) }
    }
    pub fn uniform_texture(&self, name: &str, unit: u32, value: &Texture) {
        unsafe {
            self.gl.ActiveTexture(GLenum(GL_TEXTURE0.0 + unit));
            self.gl.BindTexture(GL_TEXTURE_2D, value.id);
        }
        self.uniform1i(name, unit as _);
    }
    pub fn uniform_framebuffer(&self, name: &str, unit: u32, value: &Framebuffer) {
        unsafe {
            self.gl.ActiveTexture(GLenum(GL_TEXTURE0.0 + unit));
            self.gl.BindTexture(GL_TEXTURE_2D, value.color);
        }
        self.uniform1i(name, unit as _);
    }
}