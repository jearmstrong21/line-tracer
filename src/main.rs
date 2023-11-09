mod shader;
mod texture;
mod quad;
mod perf;
mod font;
mod emit;
mod framebuffer;

use shader::*;
use texture::*;
use quad::*;
use perf::*;
use font::*;
use emit::*;
use framebuffer::*;

use std::rc::Rc;
use gl33::*;
use glfw::{Context, OpenGlProfileHint, SwapInterval, WindowHint, WindowMode};
use ultraviolet::Vec2;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
// const WIDTH: u32 = 2400;
// const HEIGHT: u32 = 1200;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    let (mut win, rec) = glfw.create_window(WIDTH, HEIGHT, "RTX", WindowMode::Windowed).unwrap();
    win.make_current();
    glfw.set_swap_interval(SwapInterval::Sync(1));
    let gl = Rc::new(unsafe { GlFns::load_from(&|symbol| glfw::ffi::glfwGetProcAddress(symbol as _)) }.unwrap());

    // let main_shader = Shader::new(gl.clone(), "test", false);
    // let main_geom = Quad::new(gl.clone());
    let main_shader = Shader::new(gl.clone(), "emit", true);
    let main_geom = EmitGeom::new(gl.clone());

    let postprocess = Shader::new(gl.clone(), "postprocess", false);
    let framebuffer = Framebuffer::new(gl.clone());
    let quad = Quad::new(gl.clone());

    let font = Font::new(gl.clone());

    let mut perf = Perf::new();
    let mut frame_count = 0;

    while !win.should_close() {
        win.swap_buffers();
        glfw.poll_events();
        glfw::flush_messages(&rec);

        frame_count += 1;
        perf.frame();
        unsafe {
            framebuffer.start();
            gl.Viewport(0, 0, WIDTH as _, HEIGHT as _);
            gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT | GL_STENCIL_BUFFER_BIT);

            gl.Enable(GL_BLEND);
            gl.BlendFunc(GL_ONE, GL_ONE);
            gl.BlendEquation(GL_FUNC_ADD);

            main_shader.set();
            let (x, y) = win.get_cursor_pos();
            let x = x as f32;
            let y = HEIGHT as f32 - y as f32;
            let x = (x - WIDTH as f32 / 2.0) * 2.0 + WIDTH as f32 / 2.0;
            let y = (y - HEIGHT as f32 / 2.0) * 2.0 + HEIGHT as f32 / 2.0;
            main_shader.uniform2f("mouseCoords", Vec2::new(x, y));
            main_shader.uniform2f("screenSize", Vec2::new(WIDTH as _, HEIGHT as _));
            main_shader.uniform1i("frameCount", frame_count);
            main_shader.uniform1f("systemTime", glfw.get_time() as f32);
            main_geom.render();
            framebuffer.end();

            gl.Viewport(0, 0, win.get_framebuffer_size().0 as _, win.get_framebuffer_size().1 as _);
            gl.Disable(GL_BLEND);

            postprocess.set();
            postprocess.uniform2f("screenSize", Vec2::new(WIDTH as _, HEIGHT as _));
            postprocess.uniform_framebuffer("tex", 0, &framebuffer);
            quad.render();

            font.render(&["RTX", &perf.text]);
        }
    }
}