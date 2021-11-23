use std::{time::{Duration}};
use engine::{clock::Clock};
use glutin::{event_loop::{ControlFlow, EventLoop}};
use crate::{engine::{camera::{PerspectiveCamera}, objectg::ObjectG, scene::{Scene}, script::Script}, graph::{mesh::{Mesh, MeshPrimitives}, renderer::Renderer, shaders::program::Program}, renderers::{opengl::{MeshGL, OpenGL}}};

mod math;
mod graph;
mod extra;
mod engine;
mod renderers;

fn main() {
    println!();
    let (renderer, mut scene) = opengl_basic_setup("Hello world", 900, 900);
    
    let script = Script::of_update(|s : &mut Scene<OpenGL>, d| {
        let sec = d.as_secs_f32();
        let obj = &mut s.objects[0];

        obj.transform.rotate(sec, sec * 1.1, sec * 1.2);
        s.camera.rotate(0., sec / 12., 0.)
    });

    let mesh = MeshPrimitives::circle::<OpenGL, 20>(&renderer);
    let mut obj = ObjectG::new(mesh);

    obj.transform.position[2] -= 5.;
    obj.transform.set_scale(0.5);

    scene.script = script;
    scene.objects.push(obj);

    renderer.run(scene)
}

fn opengl_basic_setup (title: &str, width: u32, height: u32) -> (OpenGL, Scene<OpenGL>) {
    let renderer = OpenGL::new();    
    let window = renderer.create_window(title, width, height, false);

    let vertex = renderer.create_vertex_shader_from("./opengl/shader.vs");
    let fragment = renderer.create_fragment_shader_from("./opengl/shader.fs");
    let program = renderer.create_program(vertex, fragment, &["world_matrix", "camera"]);

    let camera = PerspectiveCamera::new((60f32).to_radians(), 0.01, 1000.);
    let scene = Scene::new(window, program, camera, vec![], Script::empty());

    (renderer, scene)
}