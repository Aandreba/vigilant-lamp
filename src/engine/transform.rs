use crate::{math::{quaternion::Quaternion32}, vector::EucVecf3, matrix::Matf4};

/// Struct representing a 3d transformation of an element
pub struct Transform {
    pub position: EucVecf3,
    pub rotation: Quaternion32,
    pub scale: EucVecf3
}

// INITS
impl Transform {
    pub fn default () -> Transform {
        Transform { position: EucVecf3::default(), rotation: Quaternion32::zero_rotation(), scale: EucVecf3::new(1., 1., 1.) }
    }

    pub fn new (position: EucVecf3, rotation: Quaternion32, scale: EucVecf3) -> Transform {
        Transform { position, rotation, scale }
    }

    pub fn of_position (position: EucVecf3) -> Transform {
        Transform { position, rotation: Quaternion32::zero_rotation(), scale: EucVecf3::new(1., 1., 1.) }
    }

    pub fn of_rotation (rotation: Quaternion32) -> Transform {
        Transform { position: EucVecf3::default(), rotation, scale: EucVecf3::new(1., 1., 1.) }
    }

    pub fn of_scale (scale: EucVecf3) -> Transform {
        Transform { position: EucVecf3::default(), rotation: Quaternion32::zero_rotation(), scale }
    }

    pub fn of_angles (roll: f32, pitch: f32, yaw: f32) -> Transform {
        Transform { position: EucVecf3::default(), rotation: Quaternion32::from_angles(roll, pitch, yaw), scale: EucVecf3::new(1., 1., 1.) }
    }
}

// METHODS
impl Transform {
    pub fn set_scale (&mut self, value: f32) {
        self.scale = EucVecf3::new(value, value, value)
    }

    pub fn rotate (&mut self, roll: f32, pitch: f32, yaw: f32) {
        self.rotation = self.rotation * Quaternion32::from_angles(roll, pitch, yaw);
        self.rotation = self.rotation.unit();
    }

    pub fn position_matrix (&self) -> Matf4 {
        Matf4::of(
            1., 0., 0., self.position.x,
            0., 1., 0., self.position.y,
            0., 0., 1., self.position.z,
            0., 0., 0., 1.
        )
    }

    pub fn scale_matrix (&self) -> Matf4 {
        Matf4::of(
            self.position.x, 0., 0., 0.,
            0., self.position.y, 0., 0.,
            0., 0., self.position.z, 0.,
            0., 0., 0., 1.
        )
    }

    pub fn matrix (&self) -> Matf4 {
        self.position_matrix() * self.rotation.rot_matrix4() * self.scale_matrix()
    }
}