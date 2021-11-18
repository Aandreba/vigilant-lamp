use crate::{graph::window::Window, math::{array_ext::NumArray, matrix::{Matrix4}, quaternion::quaternion::Quaternion32}};

pub trait Camera {
    fn get_position (&mut self) -> &mut NumArray<f32, 3>;
    fn get_rotation (&mut self) -> &mut Quaternion32;
    fn view_matrix<T: Window> (&self, window: &T) -> Matrix4<f32>;
}

// PERSPECTIVE CAMERA
struct PerspectiveCamera {
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,

    pub position: NumArray<f32, 3>,
    pub rotation: Quaternion32
}

impl Camera for PerspectiveCamera {
    fn get_position (&mut self) -> &mut NumArray<f32, 3> {
        &mut self.position
    }

    fn get_rotation (&mut self) -> &mut Quaternion32 {
        &mut self.rotation
    }

    fn view_matrix<T: Window> (&self, window: &T) -> Matrix4<f32> {
        let ar = window.get_aspect_ratio();
        let alpha = (self.fov / 2.0).tan();
        
        let zp = self.z_far + self.z_near;
        let zm = self.z_far - self.z_near;

        Matrix4::from_array([
            [1. / (ar * alpha), 0., 0., 0.],
            [0., 1. / alpha, 0., 0.],
            [0., 0., -zp / zm, -(2. * self.z_far * self.z_near) / zm],
            [0., 0., -1., 0.]
        ])
    }
}