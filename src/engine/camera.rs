use crate::{math::{matrix::{Mat4}, quaternion::Quaternion32}, vector::{EucVec2, EucVecf3}, matrix::Matf4};

/// Element used to represent the view characteristics of a scene
pub trait Camera {
    fn projection_matrix (&self, width: u32, height: u32) -> Mat4<f32>;

    fn get_position (&self) -> &EucVecf3;
    fn get_rotation (&self) -> &Quaternion32;

    fn get_position_mut (&mut self) -> &mut EucVecf3;
    fn get_rotation_mut (&mut self) -> &mut Quaternion32;

    fn set_position (&mut self, value: EucVecf3);
    fn set_rotation (&mut self, value: Quaternion32);

    fn translate (&mut self, x: f32, y: f32, z: f32) {
        self.set_position(*self.get_position() + EucVecf3::new(x, y, z))
    }

    fn rotate (&mut self, roll: f32, pitch: f32, yaw: f32) {
        let nw = *self.get_rotation() * Quaternion32::from_angles(roll, pitch, yaw);
        self.set_rotation(nw.unit())
    }

    fn view_matrix (&self) -> Matf4 {
        let position = Matf4::of(
            1., 0., 0., -self.get_position().x,
            0., 1., 0., -self.get_position().y,
            0., 0., 1., -self.get_position().z,
            0., 0., 0., 1.
        );

        self.get_rotation().rot_matrix4().transp() * position
        // position * self.get_rotation().point_rot_matrix4()
    }

    fn camera_matrix (&self, width: u32, height: u32) -> Matf4 {
        self.projection_matrix(width, height) * self.view_matrix()
    }
}

/// Camera that represents a perspective view. It's the most common type of camera, since it's the best at emulating the way human vision works
pub struct PerspectiveCamera {
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,

    pub position: EucVecf3,
    pub rotation: Quaternion32
}

impl PerspectiveCamera {
    pub fn new (fov: f32, z_near: f32, z_far: f32) -> PerspectiveCamera {
        PerspectiveCamera { fov, z_near, z_far, position: EucVecf3::default(), rotation: Quaternion32::zero_rotation() }
    }
}

impl Camera for PerspectiveCamera {
    fn get_position(&self) -> &EucVecf3 {
        &self.position
    }

    fn get_rotation(&self) -> &Quaternion32 {
        &self.rotation
    }

    fn get_position_mut (&mut self) -> &mut EucVecf3 {
        &mut self.position
    }

    fn get_rotation_mut (&mut self) -> &mut Quaternion32 {
        &mut self.rotation
    }

    fn set_position(&mut self, value: EucVecf3) {
        self.position = value
    }

    fn set_rotation(&mut self, value: Quaternion32) {
        self.rotation = value
    }

    fn projection_matrix (&self, width: u32, height: u32) -> Matf4 {
        let aspect = (width as f32) / (height as f32);
        let h = (self.fov * 0.5).tan();

        let zp = self.z_far + self.z_near;
        let zm = self.z_far - self.z_near;

        let rm00 = 1.0 / (h * aspect);
        let rm11 = 1.0 / h;

        Matf4::of(
            rm00, 0., 0., 0.,
            0., rm11, 0., 0.,
            0., 0., -zp / zm, -2. * self.z_far * self.z_near / zm,
            0., 0., -1., 0.
        )
    }
}