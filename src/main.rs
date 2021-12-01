use crate::{engine::{camera::{PerspectiveCamera}, input::{keyboard::{KeyboardKey, KeyboardListener}, mouse::MouseListener}, objectg::ObjectG, scene::{Scene}, script::Script}, extra::color::Color, graph::{mesh::{CUBE_INDICES, CUBE_VERTICES, MeshPrimitives}, renderer::Renderer}, math::quaternion::Quaternion32, renderers::{opengl::{OpenGL}}};

mod math;
mod graph;
mod extra;
mod engine;
mod renderers;

fn main() {
    println!();

    let (mut renderer, mut scene) = opengl_basic_setup("Hello world", 900, 900);
    
    let script = Script::of_update(|s : &mut Scene<OpenGL>, k, m, d| {
        let sec = d.as_secs_f32();
        let obj = &mut s.objects[0];

        if k.is_pressed(KeyboardKey::ESCAPE) {
            panic!()
        }

        if k.is_pressed(KeyboardKey::W) {
            s.camera.translate(0., 0., -sec)
        } if k.is_pressed(KeyboardKey::A) {
            s.camera.translate(-sec, 0., 0.)
        } if k.is_pressed(KeyboardKey::S) {
            s.camera.translate(0., 0., sec)
        } if k.is_pressed(KeyboardKey::D) {
            s.camera.translate(sec, 0., 0.)
        } if k.is_pressed(KeyboardKey::SPACE) {
            s.camera.translate(0., sec, 0.)
        } if k.is_pressed(KeyboardKey::LEFT_SHIFT) {
            s.camera.translate(0., -sec, 0.)
        }

        let mouse = m.relative_position();
        println!("{}", mouse);

        s.camera.set_rotation(Quaternion32::from_angles(mouse.y(), mouse.x(), 0.))
    });

    let mesh = MeshPrimitives::cube(&renderer);
    let mut obj = ObjectG::new(mesh);

    obj.transform.position[2] -= 5.;
    obj.transform.set_scale(0.5);

    scene.script = script;
    scene.objects.push(obj);

    renderer.set_wireframe(true);
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