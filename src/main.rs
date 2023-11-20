mod shader;
mod perf;
mod geometry;

use std::mem;
use std::time::SystemTime;
use egui::{CollapsingHeader, Color32, Frame, Margin, RichText, SidePanel, Slider, Ui};
use glium::{Blend, BlendingFunction, DrawParameters, LinearBlendingFactor, Surface, Texture2d, uniform};
use glium::backend::glutin::SimpleWindowBuilder;
use glium::framebuffer::SimpleFrameBuffer;
use glium::index::{IndicesSource, PrimitiveType};
use glium::texture::{MipmapsOption, UncompressedFloatFormat};
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use shader::*;
use perf::*;
use geometry::*;

const WIDTH: u32 = 2400;
const HEIGHT: u32 = 1600;

fn main() {
    let start_time = SystemTime::now();

    let event_loop = EventLoopBuilder::new().build();
    let (window, display) = SimpleWindowBuilder::new()
        .with_inner_size(WIDTH, HEIGHT)
        .with_title("line-tracer")
        .build(&event_loop);

    window.set_resizable(false);

    println!("{} {} {}", display.get_opengl_version_string(), display.get_opengl_vendor_string(), display.get_opengl_renderer_string());

    let main_shader = load_shader(&display, "emit", true);
    let main_vertex = emit_geom(&display, 10000);

    let postprocess = load_shader(&display, "postprocess", false);
    let geom_overlay = load_shader(&display, "geom", false);

    let render_target = Texture2d::empty_with_format(&display,
                                                     UncompressedFloatFormat::F16F16F16F16,
                                                     MipmapsOption::NoMipmap,
                                                     WIDTH, HEIGHT).unwrap();

    let (quad_vertex, quad_index) = quad_geom(&display);

    let mut perf = Perf::new();

    let mut mouse_engaged = true;
    let mut draw_geom = true;

    let mut egui = egui_glium::EguiGlium::new(&display, &window, &event_loop);

    let mut scene_data = SceneData {
        arcs: unsafe { mem::zeroed() },
        circles: unsafe { mem::zeroed() },
        lines: unsafe { mem::zeroed() },
        arc_count: 0,
        circle_count: 0,
        line_count: 0,
        light_origin: [0.2f32 * WIDTH as f32, 0.2f32 * HEIGHT as f32],
        light_angle: 0.3,
        bounce_diminish: 1.0,
        mouse_coords: [0.0, 0.0],
        screen_size: [WIDTH as f32, HEIGHT as f32],
        frame_count: 0,
        system_time: 0.0,
        eta_a: 0.9,
        eta_b: 0.4,
    };

    {
        let w = WIDTH as f32;
        let h = HEIGHT as f32;

        fn line(x0: impl Into<f32>, y0: impl Into<f32>, x1: impl Into<f32>, y1: impl Into<f32>, m: i32) -> Line {
            Line {
                a: [x0.into(), y0.into()],
                b: [x1.into(), y1.into()],
                m
            }
        }
        /*
800.0, 300.0), vec2(1000.0, 300.0), M_REFR), hr);
//    hitLine(r, Line(vec2(900.0, 500.0), vec2(1000.0, 300.0), M_REFR), hr);
//    hitLine(r, Line(vec2(900.0, 500.0), vec2(800.0, 300.0), M_REFR), hr)
 */
        let mut al = |l: Line| {
            scene_data.lines[scene_data.line_count as usize] = l;
            scene_data.line_count += 1;
        };

        al(line(0., 0., w, 0., MATERIAL_REFLECT)); // right
        al(line(w, 0., w, h, MATERIAL_REFLECT)); // bottom
        al(line(0., h, w, h, MATERIAL_REFLECT)); // top

        al(line(1600.0, 600.0, 2000.0, 600.0, MATERIAL_REFRACT));
        al(line(1800.0, 1000.0, 2000.0, 600.0, MATERIAL_REFRACT));
        al(line(1800.0, 1000.0, 1600.0, 600.0, MATERIAL_REFRACT));
    }

    event_loop.run(move |event, _, control_flow| {
        perf.frame();

        match event {
            Event::WindowEvent { event, .. } => {
                let response = egui.on_event(&event);
                if !response.consumed {
                    match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            if mouse_engaged {
                                scene_data.mouse_coords = [position.x as f32, HEIGHT as f32 - position.y as f32];
                            }
                        },
                        WindowEvent::KeyboardInput { input, .. } => {
                            if input.state == ElementState::Pressed && input.virtual_keycode == Some(VirtualKeyCode::Return) {
                                mouse_engaged = !mouse_engaged;
                            }
                        },
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::ExitWithCode(0);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }


        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let mut framebuffer = SimpleFrameBuffer::new(&display, &render_target).unwrap();
        framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);
        framebuffer.draw(
            &main_vertex,
            IndicesSource::NoIndices { primitives: PrimitiveType::Points },
            &main_shader,
            &scene_data,
            &DrawParameters {
                blend: Blend {
                    color: BlendingFunction::Addition { source: LinearBlendingFactor::One, destination: LinearBlendingFactor::One },
                    alpha: BlendingFunction::Addition { source: LinearBlendingFactor::One, destination: LinearBlendingFactor::One },
                    constant_value: (0.0, 0.0, 0.0, 0.0),
                },
                ..Default::default()
            }
        ).unwrap();

        if draw_geom {
            let (overlay_vertex, overlay_index) = scene_data.overlay_geom(&display);
            framebuffer.draw(
                &overlay_vertex,
                &overlay_index,
                &geom_overlay,
                &uniform! {
                screen_size: scene_data.screen_size,
                color: [1.0f32, 1.0, 1.0]
            },
                &Default::default()
            ).unwrap();
        }

        target.draw(&quad_vertex,
                    &quad_index,
                    &postprocess,
                    &uniform! {
                        screenSize: [WIDTH as f32, HEIGHT as f32],
                        tex: &render_target
                    },
                    &Default::default()
        ).unwrap();


        _ = egui.run(&window, |ctx| {
            SidePanel::left("panel").resizable(false).min_width(250.0).frame(Frame::default().inner_margin(Margin::same(10.0)).fill(Color32::BLACK).multiply_with_opacity(0.4)).show(ctx, |ui| {
                ui.heading("Line tracer");
                ui.label(format!("FPS: {}", perf.text));
                ui.label(RichText::new(if mouse_engaged { "Mouse engaged" } else { "Mouse disengaged" }).color(if mouse_engaged { Color32::GREEN } else { Color32::RED }));
                ui.checkbox(&mut draw_geom, "Draw geometry overlay");
                ui.add(Slider::new(&mut scene_data.bounce_diminish, 0.0..=1.0).text("Bounce diminish"));

                let xrange = 0f32..=WIDTH as f32;
                let yrange = 0f32..=HEIGHT as f32;
                CollapsingHeader::new("Lighting").default_open(true).show(ui, |ui| {
                    ui.add(Slider::new(&mut scene_data.light_origin[0], xrange.clone()).text("x"));
                    ui.add(Slider::new(&mut scene_data.light_origin[1], yrange.clone()).text("y"));
                    ui.add(Slider::new(&mut scene_data.light_angle, 0f32..=std::f32::consts::TAU).text("theta"));
                    ui.add(Slider::new(&mut scene_data.eta_a, 0f32..=1.5).text("eta a"));
                    ui.add(Slider::new(&mut scene_data.eta_b, 0f32..=1.5).text("eta b"));
                });

                fn material_radio(ui: &mut Ui, m: &mut i32) {
                    if ui.radio(*m == MATERIAL_DIFFUSE, "diffuse").clicked() {
                        *m = MATERIAL_DIFFUSE;
                    }
                    if ui.radio(*m == MATERIAL_REFLECT, "reflect").clicked() {
                        *m = MATERIAL_REFLECT;
                    }
                    if ui.radio(*m == MATERIAL_REFRACT, "refract").clicked() {
                        *m = MATERIAL_REFRACT;
                    }
                }

                CollapsingHeader::new("Circles").default_open(true).show(ui, |ui| {
                    let mut delete_circle = None;
                    for i in 0..scene_data.circle_count as usize {
                        CollapsingHeader::new(format!("Circle {}", i)).show(ui, |ui| {
                            if ui.button("-").clicked() {
                                delete_circle = Some(i);
                            }
                            ui.add(Slider::new(&mut scene_data.circles[i].p[0], xrange.clone()).text("x"));
                            ui.add(Slider::new(&mut scene_data.circles[i].p[1], yrange.clone()).text("y"));
                            ui.add(Slider::new(&mut scene_data.circles[i].r, 0.0..=400.0).text("r"));
                            material_radio(ui, &mut scene_data.circles[i].m);
                        });
                    }
                    if let Some(i) = delete_circle {
                        for j in i + 1..scene_data.circle_count as usize {
                            scene_data.circles[j - 1] = scene_data.circles[j];
                        }
                        scene_data.circle_count -= 1;
                    }
                    if ui.button("+").clicked() {
                        if scene_data.circle_count < MAX_CIRCLE_COUNT as i32 {
                            scene_data.circles[scene_data.circle_count as usize] = Circle {
                                p: [0.5 * WIDTH as f32, 0.5 * HEIGHT as f32],
                                r: 200.0,
                                m: MATERIAL_REFRACT,
                            };
                            scene_data.circle_count += 1;
                        }
                    }
                });

                CollapsingHeader::new("Lines").default_open(true).show(ui, |ui| {
                    let mut delete_line = None;
                    for i in 0..scene_data.line_count as usize {
                        CollapsingHeader::new(format!("Line {}", i)).show(ui, |ui| {
                            if ui.button("-").clicked() {
                                delete_line = Some(i);
                            }
                            ui.add(Slider::new(&mut scene_data.lines[i].a[0], xrange.clone()).text("x0"));
                            ui.add(Slider::new(&mut scene_data.lines[i].a[1], yrange.clone()).text("y0"));
                            ui.add(Slider::new(&mut scene_data.lines[i].b[0], xrange.clone()).text("x1"));
                            ui.add(Slider::new(&mut scene_data.lines[i].b[1], yrange.clone()).text("y1"));
                            material_radio(ui, &mut scene_data.lines[i].m);
                        });
                    }
                    if let Some(i) = delete_line {
                        for j in i + 1..scene_data.line_count as usize {
                            scene_data.lines[j - 1] = scene_data.lines[j];
                        }
                        scene_data.line_count -= 1;
                    }
                    if ui.button("+").clicked() {
                        if scene_data.line_count < MAX_LINE_COUNT as i32 {
                            scene_data.lines[scene_data.line_count as usize] = Line {
                                a: [0.3 * WIDTH as f32, 0.5 * HEIGHT as f32],
                                b: [0.7 * WIDTH as f32, 0.5 * HEIGHT as f32],
                                m: MATERIAL_REFRACT,
                            };
                            scene_data.line_count += 1;
                        }
                    }
                });
            });
        });
        egui.paint(&display, &mut target);

        target.finish().unwrap();
    });
}