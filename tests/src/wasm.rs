use vigilant_lamp::quaternion::Quaternion32;
use vigilant_lamp::{Script, Scene, Window};
use vigilant_lamp::webgl::{WindowWGL, MeshWGL};
use wasm_bindgen::prelude::*;
use vigilant_lamp::extra::wasm_mappings::*;
use vigilant_lamp::renderers::webgl::WebGL;
use vigilant_lamp::{builder::build_webgl, PerspectiveCamera, MeshPrimitives, Mesh, ObjectG, Renderer};
use web_sys::console::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use vigilant_lamp::{Material, Color};

    println!();
    map_panic();

    let build : Result<(WebGL, Scene<WebGL>), JsValue> = build_webgl("#gl", default_cam());
    match build {
        Err(x) => {
            println!("{:?}", x);
            Err(x)
        },
        Ok((renderer, mut scene)) => {
            scene.script = default_script();
        
            let cube = MeshPrimitives::cube(&renderer);
            panic!("{:?}", cube.unwrap().get_normals());

            let material = Material::of_color(Color::YELLOW);
            let object : Result<ObjectG<WebGL>, JsValue> = cube.map(|x| ObjectG::of_mesh(x, material));
            
            match object {
                Err(x) => {
                    println!("{:?}", x);
                    Err(x)
                },
                Ok(mut object) => {
                    object.transform.position.z = -5.;
                    object.transform.set_scale(0.5);
                
                    scene.objects.push(object);
                    println!("{:?}", scene.window.get_property("scroll_y"));

                    renderer.run(scene);
                    Ok(())
                }
            }
        }
    }
}

fn default_cam () -> PerspectiveCamera {
    PerspectiveCamera::new(60f32.to_radians(), 0.01, 1000.)
}

fn default_script<R: Renderer> () -> Script<R> {
    Script::<R>::of_update(|s, k, m, d| {
        let sec = d.as_secs_f32();
        let obj = &mut s.objects[0];

        //obj.transform.rotate(sec, sec * 1.1, sec * 1.2);
        let scroll = s.window.get_property_copy_as::<f64>("scroll_y");

        match scroll {
            None => (),
            Some(x) => {
                let pct = (x as f32) / (s.window.get_height() as f32);
                obj.transform.rotation = Quaternion32::from_angles(pct, -pct * 1.1, pct * 1.2)
            }
        }
    })
}