use std::{time::{Duration}};
use engine::{clock::Clock};
use glutin::{event_loop::{ControlFlow, EventLoop}};
use crate::{engine::{camera::{PerspectiveCamera}, objectg::ObjectG, scene::{Scene}, script::Script}, graph::{mesh::Mesh, renderer::Renderer, shaders::program::Program}, renderers::{opengl::{MeshGL, OpenGL}}};

mod math;
mod graph;
mod extra;
mod engine;
mod renderers;

fn main() {
    println!();

    let renderer = OpenGL::new();    
    let window = renderer.create_window("Hello world", 900, 900, true);
    let square : MeshGL = Mesh::square::<OpenGL>(&renderer);

    let vertex = renderer.create_vertex_shader_from("./opengl/shader.vs");
    let fragment = renderer.create_fragment_shader_from("./opengl/shader.fs");
    let program = renderer.create_program(vertex, fragment, &["world_matrix"]);

    let camera = PerspectiveCamera::new((60f32).to_radians(), 0.01, 1000.);
    let object = ObjectG::new(square);

    let mut scene = Scene::new(window, program, camera, vec![object], Script::empty());
    let script = Script::of_update(|s: &mut Scene<OpenGL>, d| {
        s.objects[0].transform.rotate(0., 0., d.as_secs_f32())
    });

    scene.script = script;
    renderer.run(scene)
}