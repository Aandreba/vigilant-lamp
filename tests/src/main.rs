use vigilant_lamp::input::{KeyboardListener, KeyboardKey};
use vigilant_lamp::{Script, Scene};
use vigilant_lamp::{builder::build_opengl, PerspectiveCamera, MeshPrimitives, ObjectG, Renderer};

pub fn main () {
    let gl = build_opengl("Hello world", 900, 900, false, default_cam());
    
    match gl {
        Err(x) => panic!("Error: {}", x),
        Ok((renderer, mut scene)) => {
            match default_scene(&renderer, &mut scene) {
                Err(x) => panic!("Error: {}", x),
                Ok(_) => println!("Scene loaded")
            }

            scene.script = default_script();
            match renderer.run(scene) {
                Err(x) => panic!("Error: {}", x),
                Ok(_) => println!("Done")
            }
        }
    }
}

fn default_cam () -> PerspectiveCamera {
    PerspectiveCamera::new(60f32.to_radians(), 0.01, 1000.)
}

fn default_scene<R: Renderer> (renderer: &R, scene: &mut Scene<R>) -> Result<(), R::ErrorType> {
    let mesh = MeshPrimitives::cube(renderer);
    match mesh {
        Err(x) => Err(x),
        Ok(mesh) => {
            let mut obj = ObjectG::new(mesh);

            obj.transform.position[2] = -5.;
            obj.transform.set_scale(0.5);
            
            scene.objects.push(obj);
            Ok(())
        }
    }
}

fn default_script<R: Renderer> () -> Script<R> {
    Script::<R>::of_update(|s, k, m, d| {
        let sec = d.as_secs_f32();
        let obj = &mut s.objects[0];

        obj.transform.rotate(sec, sec * 1.1, sec * 1.2);

        if k.is_pressed(KeyboardKey::ESCAPE) {
            println!("Death");
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

        println!("{}", 1. / sec)
        //let mouse = m.relative_position();
        //s.camera.set_rotation(Quaternion32::from_angles(-mouse.y(), -mouse.x(), 0.))
    })
}