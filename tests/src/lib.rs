use vigilant_lamp::{wasm, desktop};

wasm! {
    use vigilant_lamp::{Script, Scene};,
    use vigilant_lamp::webgl::{WindowWGL, MeshWGL};,
    use wasm_bindgen::prelude::*;,
    use vigilant_lamp::extra::wasm_mappings::*;,
    use vigilant_lamp::renderers::webgl::WebGL;,
    use vigilant_lamp::{builder::build_webgl, PerspectiveCamera, MeshPrimitives, ObjectG, Renderer};
}

desktop! {
    include!("main.rs");
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    println!();
    map_panic();

    let camera = PerspectiveCamera::new((60f32).to_radians(), 0.01, 1000.);
    let build : Result<(WebGL, Scene<WebGL>), JsValue> = build_webgl("#gl", camera);
    match build {
        Err(x) => {
            println!("{:?}", x);
            Err(x)
        },
        Ok((renderer, mut scene)) => {
            scene.script = get_script();
        
            let cube = MeshPrimitives::cube(&renderer);
            let object : Result<ObjectG<MeshWGL>, JsValue> = cube.map(|x| ObjectG::new(x));
            
            match object {
                Err(x) => {
                    println!("{:?}", x);
                    Err(x)
                },
                Ok(mut object) => {
                    object.transform.position[2] = -5.;
                    object.transform.set_scale(0.5);
                
                    scene.objects.push(object);
                    println!("{:?}", renderer.get_property("scroll_y"));

                    renderer.run(scene);
                    Ok(())
                }
            }
        }
    }
}