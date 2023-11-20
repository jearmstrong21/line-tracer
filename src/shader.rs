use std::fs;
use glium::{Display, Program, ProgramCreationError};
use glium::glutin::surface::WindowSurface;
use glium::program::ShaderType;

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
            // cur = format!("///////// START {}\n", include);
            cur += &shader_source(&include, "glsl", already_included);
            // cur = format!("///////// END {}\n", include);
        } else {
            cur = format!("{}{}", cur, raw.chars().nth(0).unwrap());
            raw = (&raw[1..]).to_string();
        }
    }
    return cur
}

pub fn load_shader(display: &Display<WindowSurface>, name: &str, has_geom: bool) -> Program {
    let vert = shader_source(name, "vert", &mut vec![]);
    let frag = shader_source(name, "frag", &mut vec![]);
    let mut geom = None;
    let result = if has_geom {
        geom = Some(shader_source(name, "geom", &mut vec![]));
        Program::from_source(display, &vert, &frag, geom.as_ref().map(|s| s.as_str()))
    } else {
        Program::from_source(display, &vert, &frag, None)
    };
    match result {
        Ok(program) => program,
        Err(error) => {
            println!("SHADER ERROR\n{:?}", error);
            match error {
                ProgramCreationError::CompilationError(error, stage) => {
                    let src = match stage {
                        ShaderType::Vertex => vert,
                        ShaderType::Geometry => geom.unwrap_or("none".to_string()),
                        ShaderType::Fragment => frag,
                        _ => "none".to_string()
                    };
                    fs::write("./error_shader.glsl", src).unwrap();
                    println!("In stage {:?}:\n{}", stage, error);
                }
                ProgramCreationError::LinkingError(_) => {}
                ProgramCreationError::ShaderTypeNotSupported => {}
                ProgramCreationError::CompilationNotSupported => {}
                ProgramCreationError::TransformFeedbackNotSupported => {}
                ProgramCreationError::PointSizeNotSupported => {}
                ProgramCreationError::BinaryHeaderError => {}
            }
            panic!()
        }
    }
}