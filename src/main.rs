use std::time::SystemTime;

use crate::{engine::{camera::{PerspectiveCamera}, clock::Clock, objectg::ObjectG, scene::Scene}, graph::{mesh::Mesh, renderer::Renderer, window::Window}, renderers::{opengl::OpenGL}};

mod math;
mod graph;
mod extra;
mod engine;
mod renderers;

fn main() {
    println!();

    let renderer = OpenGL::new();    
    let window = renderer.create_window("Hello world", 900, 900, true);

    let vertex = renderer.create_vertex_shader_from("./opengl/shader.vs");
    let fragment = renderer.create_fragment_shader_from("./opengl/shader.fs");
    let program = renderer.create_program(vertex, fragment);

    let camera = PerspectiveCamera::new((60f32).to_radians(), 0.01, 1000.);
    let triangle = renderer.create_mesh(&[[-0.5, -0.5, 0.], [0.5, -0.5, 0.], [0., 0.5, 0.]], &[[0, 1, 2]]);
    let object = ObjectG::new(triangle);

    let scene = Scene::new(renderer, window, program, camera, vec![object]);
    scene.run()
}