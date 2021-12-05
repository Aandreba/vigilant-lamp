#[wasm_bindgen(start)]
pub fn main() {
    println!();
    map_panic();

    let (mut renderer, window) = WebGL::new("#gl");
    let mut scene = basic_setup(&renderer, window);

    let script = Script::of_update(|s : &mut Scene<WebGL>, k, m, d| {
        let sec = d.as_secs_f32();
        let obj = &mut s.objects[0];

        obj.transform.rotate(sec, sec * 1.1, sec * 1.2);

        if k.is_pressed(KeyboardKey::ESCAPE) {
            panic!()
        } if k.is_pressed(KeyboardKey::W) {
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
        s.camera.set_rotation(Quaternion32::from_angles(-mouse.y(), -mouse.x(), 0.))
    });

    let mesh = MeshPrimitives::cube(&renderer);
    let mut obj = ObjectG::new(mesh);

    obj.transform.position[2] -= 5.;
    obj.transform.set_scale(0.5);

    scene.script = script;
    scene.objects.push(obj);

    renderer.set_wireframe(true);
    renderer.run(scene);
}

fn basic_setup<R: Renderer> (renderer: &R, window: R::WindowType) -> Scene<R> {
    let vertex_inc = include_str!("../apis/webgl/shader.vs");
    let fragment_inc = include_str!("../apis/webgl/shader.fs");

    let vertex = renderer.create_vertex_shader(vertex_inc);
    let fragment = renderer.create_fragment_shader(fragment_inc);
    let program = renderer.create_program(vertex, fragment, &["world_matrix", "camera"]);

    let camera = PerspectiveCamera::new((60f32).to_radians(), 0.01, 1000.);
    let scene = Scene::new(window, program, camera, vec![], Script::empty());

    scene
}