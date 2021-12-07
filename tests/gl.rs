#[cfg(test)]
mod tests {
    use vigilant_lamp::{builder::{build_opengl}, Scene, Renderer, PerspectiveCamera, MeshPrimitives, ObjectG, Script, FlatMap, opengl::OpenGL};

    #[test]
    fn main() {
        let gl = build_opengl("Hello world", 900, 900, true, default_camera());
        
        match gl {
            Err(x) => panic!("{}", x),
            Ok((renderer, mut scene)) => {
                let test = fill_scene(&renderer, &mut scene)
                    .flat_map_single(|_| renderer.run(scene));
                
                match test {
                    Err(x) => panic!("{}", x),
                    _ => ()
                }
            }
        }
    }

    fn default_camera () -> PerspectiveCamera {
        PerspectiveCamera::new(60f32.to_radians(), 0.01, 1000.)
    }

    fn default_script<R: Renderer> () -> Script<R> {
        Script::of_update(|s : &mut Scene<R>, k, m, d| {
            let sec = d.as_secs_f32();
            let obj = &mut s.objects[0];
            obj.transform.rotate(sec, sec * 1.1, sec * 1.2);
        })
    }

    fn fill_scene<R: Renderer> (renderer: &R, scene: &mut Scene<R>) -> Result<(), R::ErrorType> {
        let mesh = MeshPrimitives::cube(renderer);
        let obj = mesh.map(|x| ObjectG::new(x));
        match obj {
            Err(x) => Err(x),
            Ok(mut obj) => {
                obj.transform.position[2] -= 5.;
                obj.transform.set_scale(0.5);

                scene.script = default_script();
                scene.objects.push(obj);
                Ok(())
            }
        }
    }
}