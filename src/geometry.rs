use std::f32::consts::TAU;
use glium::{Display, implement_vertex, IndexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::PrimitiveType;
use glium::uniforms::{Uniforms, UniformValue};
use glium::VertexBuffer;

pub const MATERIAL_DIFFUSE: i32 = 1;
pub const MATERIAL_REFLECT: i32 = 2;
pub const MATERIAL_REFRACT: i32 = 3;


pub const MAX_ARC_COUNT: usize = 20;
pub const MAX_CIRCLE_COUNT: usize = 20;
pub const MAX_LINE_COUNT: usize = 20;

#[derive(Copy, Clone)]
pub struct Arc {
    pub p: [f32; 2],
    pub r: f32,
    pub ta: f32,
    pub tb: f32,
    pub m: i32
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub p: [f32; 2],
    pub r: f32,
    pub m: i32
}

#[derive(Copy, Clone)]
pub struct Line {
    pub a: [f32; 2],
    pub b: [f32; 2],
    pub m: i32
}

pub struct SceneData {
    pub arcs: [Arc; MAX_ARC_COUNT],
    pub circles: [Circle; MAX_CIRCLE_COUNT],
    pub lines: [Line; MAX_LINE_COUNT],
    pub arc_count: i32,
    pub circle_count: i32,
    pub line_count: i32,
    pub light_origin: [f32; 2],
    pub light_angle: f32,

    pub bounce_diminish: f32,
    pub mouse_coords: [f32; 2],
    pub screen_size: [f32; 2],
    pub frame_count: i32,
    pub system_time: f32,

    pub eta_a: f32,
    pub eta_b: f32
}

fn flatten<T, S, F: Fn(&T) -> S>(arr: &[T], get: F) -> Vec<S> {
    arr.iter().map(get).collect()
}

fn arr<'a, T, G: Fn(&T) -> UniformValue<'a>, F: FnMut(&str, UniformValue<'a>)>(name: &str, arr: &[T], get: G, f: &mut F) {
    for i in 0..arr.len() {
        f(&format!("{}[{}]", name, i), get(&arr[i]))
    }
}

impl Uniforms for SceneData {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("arc_count", UniformValue::SignedInt(self.arc_count));
        f("circle_count", UniformValue::SignedInt(self.circle_count));
        f("line_count", UniformValue::SignedInt(self.line_count));

        arr("arc_p", &self.arcs, |a| UniformValue::Vec2(a.p), &mut f);
        arr("arc_r", &self.arcs, |a| UniformValue::Float(a.r), &mut f);
        arr("arc_ta", &self.arcs, |a| UniformValue::Float(a.ta), &mut f);
        arr("arc_tb", &self.arcs, |a| UniformValue::Float(a.tb), &mut f);
        arr("arc_m", &self.arcs, |a| UniformValue::SignedInt(a.m), &mut f);

        arr("circle_p", &self.circles, |c| UniformValue::Vec2(c.p), &mut f);
        arr("circle_r", &self.circles, |c| UniformValue::Float(c.r), &mut f);
        arr("circle_m", &self.circles, |c| UniformValue::SignedInt(c.m), &mut f);

        arr("line_a", &self.lines, |l| UniformValue::Vec2(l.a), &mut f);
        arr("line_b", &self.lines, |l| UniformValue::Vec2(l.b), &mut f);
        arr("line_m", &self.lines, |l| UniformValue::SignedInt(l.m), &mut f);

        f("light_origin", UniformValue::Vec2(self.light_origin));
        f("light_angle", UniformValue::Float(self.light_angle));

        f("bounce_diminish", UniformValue::Float(self.bounce_diminish));
        f("mouse_coords", UniformValue::Vec2(self.mouse_coords));
        f("screen_size", UniformValue::Vec2(self.screen_size));
        f("frame_count", UniformValue::SignedInt(self.frame_count));
        f("system_time", UniformValue::Float(self.system_time));

        f("eta_a", UniformValue::Float(self.eta_a));
        f("eta_b", UniformValue::Float(self.eta_b));
    }
}

impl SceneData {
    pub fn overlay_geom(&self, display: &Display<WindowSurface>) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
        let mut vertices = vec![];
        let mut indices = vec![];
        let lod = 50;
        for a in &self.arcs {
            for i in 0..lod {
                let ta = (i as f32 / lod as f32) * (a.tb - a.ta) + a.ta;
                let tb = ta + (1.0 / lod as f32) * (a.tb - a.ta) + ta;
                vertices.push(Vertex { position: [a.p[0] + a.r * ta.cos(), a.p[1] + a.r * ta.sin()]});
                vertices.push(Vertex { position: [a.p[0] + a.r * tb.cos(), a.p[1] + a.r * tb.sin()]});
                indices.push(vertices.len() as u16 - 2);
                indices.push(vertices.len() as u16 - 1);
            }
        }
        for c in &self.circles {
            for i in 0..lod {
                let ta = (i as f32 / lod as f32) * TAU;
                let tb = ta + (1.0 / lod as f32) * TAU;
                vertices.push(Vertex { position: [c.p[0] + c.r * ta.cos(), c.p[1] + c.r * ta.sin()]});
                vertices.push(Vertex { position: [c.p[0] + c.r * tb.cos(), c.p[1] + c.r * tb.sin()]});
                indices.push(vertices.len() as u16 - 2);
                indices.push(vertices.len() as u16 - 1);
            }
        }
        for l in &self.lines {
            vertices.push(Vertex { position: l.a });
            vertices.push(Vertex { position: l.b });
            indices.push(vertices.len() as u16 - 2);
            indices.push(vertices.len() as u16 - 1);
        }
        (
            VertexBuffer::new(display, &vertices).unwrap(),
            IndexBuffer::new(display, PrimitiveType::LinesList, &indices).unwrap()
        )
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2]
}
implement_vertex!(Vertex, position);

pub fn emit_geom(display: &Display<WindowSurface>, n: u32) -> VertexBuffer<Vertex> {
    let mut vertices: Vec<Vertex> = vec![];
    for i in 0..n {
        let x = i as f32 / n as f32;
        vertices.push(Vertex { position: [x, x * x - 5.0 * x] });
    }

    VertexBuffer::new(display, &vertices).unwrap()
}

pub fn quad_geom(display: &Display<WindowSurface>) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
    (VertexBuffer::new(display, &vec![
        Vertex { position: [0.0, 0.0] },
        Vertex { position: [1.0, 0.0] },
        Vertex { position: [0.0, 1.0] },
        Vertex { position: [1.0, 1.0] }
    ]).unwrap(), IndexBuffer::new(display, PrimitiveType::TrianglesList, &[0, 1, 2, 1, 2, 3]).unwrap())
}