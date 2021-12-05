use crate::{Renderer, Camera, Scene, Script};

#[cfg(target_family = "wasm")]
use crate::renderers::webgl::WebGL;

#[cfg(not(target_family = "wasm"))]
use crate::{opengl::OpenGL};

#[cfg(not(target_family = "wasm"))]
pub fn build_opengl<C: Camera + 'static> (title: &str, width: u32, height: u32, vsync: bool, camera: C) -> (OpenGL, Scene<OpenGL>) {
    let renderer = OpenGL::new();
    let window = renderer.create_window(title, width, height, vsync);

    let vertex_inc = include_str!("../../apis/opengl/shader.vs");
    let fragment_inc = include_str!("../../apis/opengl/shader.fs");

    let program = build_gl(vertex_inc, fragment_inc, &renderer);
    let scene = Scene::new(window, program, camera, vec![], Script::empty());

    (renderer, scene)
}

#[cfg(target_family = "wasm")]
pub fn build_webgl<C: Camera + 'static> (selector: &str, camera: C) -> (WebGL, Scene<WebGL>) {
    let (renderer, window) = WebGL::new(selector);

    let vertex_inc = include_str!("../../apis/webgl/shader.vs");
    let fragment_inc = include_str!("../../apis/webgl/shader.fs");

    // TODO
    let program = build_gl(vertex_inc, fragment_inc, &renderer);
    let scene = Scene::new(window, program, camera, vec![], Script::empty());

    (renderer, scene)
}

fn build_gl<R: Renderer> (vertex_inc: &str, fragment_inc: &str, renderer: &R) -> R::ProgramType {
    let vertex = renderer.create_vertex_shader(vertex_inc);
    let fragment = renderer.create_fragment_shader(fragment_inc);
    renderer.create_program(vertex, fragment, &["world_matrix", "camera"])
}