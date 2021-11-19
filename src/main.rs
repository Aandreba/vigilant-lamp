use std::fmt::Debug;

use crate::{engine::{camera::{Camera, PerspectiveCamera}, scene::Scene}, graph::{debug::DebugRenderer, renderer::Renderer}};

mod engine;
mod graph;
mod math;
mod extra;

fn main() {
    println!();

    let renderer = DebugRenderer;
    let window = renderer.create_window("Hello world", 900, 900);
    let camera = PerspectiveCamera::new(f32::to_radians(60.), 0.01, 1000.);
    
    let scene = Scene::new(window, camera);
    println!("{}", scene.view_matrix())
}