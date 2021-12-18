use crate::{Renderer, Camera, Scene, Script, wasm};

wasm! {
    use crate::renderers::webgl::WebGL;,
    use wasm_bindgen::JsValue;
}

#[cfg(not(target_family = "wasm"))]
use crate::{opengl::OpenGL};

#[cfg(not(target_family = "wasm"))]
pub fn build_opengl<C: Camera + 'static> (title: &str, width: u32, height: u32, vsync: bool, camera: C) -> Result<(OpenGL, Scene<OpenGL>), String> {
    let renderer = OpenGL::new();
    let window = renderer.create_window(title, width, height, vsync);
   
    match window {
        Err(x) => return Err(x),
        Ok(window) => {
            let vertex_inc = include_str!("../../apis/opengl/shader.vs");
            let fragment_inc = include_str!("../../apis/opengl/shader.fs");

            let program = build_gl(vertex_inc, fragment_inc, &renderer);
            match program {
                Err(x) => return Err(x),
                Ok(program) => {
                    let scene = Scene::new(window, program, camera, vec![], Script::empty());
                    Ok((renderer, scene))
                }
            }
        }
    }
}

#[cfg(target_family = "wasm")]
pub fn build_webgl<C: Camera + 'static> (selector: &str, camera: C) -> Result<(WebGL, Scene<WebGL>), JsValue> {
    let webgl = WebGL::new(selector);
    match webgl {
        Err(x) => Err(x),
        Ok((renderer, window)) => {
            let vertex_inc = include_str!("../../apis/webgl/shader.vs");
            let fragment_inc = include_str!("../../apis/webgl/shader.fs");

            let program = build_gl(vertex_inc, fragment_inc, &renderer);
            match program {
                Err(x) => Err(x),
                Ok(program) => {
                    let scene = Scene::new(window, program, camera, vec![], Script::empty());
                    Ok((renderer, scene))
                }
            }
        }
    }
}

fn build_gl<R: Renderer> (vertex_inc: &str, fragment_inc: &str, renderer: &R) -> Result<R::ProgramType, R::ErrorType> {
    let vertex = renderer.create_vertex_shader(vertex_inc);
    match vertex {
        Err(x) => Err(x),
        Ok(x) => {
            let fragment = renderer.create_fragment_shader(fragment_inc);
            match fragment {
                Err(z) => Err(z),
                Ok(z) => renderer.create_program(x, z, &[
                    "world_matrix", "camera", 
                    "ambient.color", "ambient.intensity", 
                    "point.position", "point.light.color", "point.light.intensity",
                    "material.color", "material.texture"
                ])
            }
        }
    }
}