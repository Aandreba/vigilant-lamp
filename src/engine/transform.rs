use crate::{math::{array_ext::NumArray, matrix::{Matrix4}, quaternion::Quaternion32}};

pub struct Transform {
    pub position: NumArray<f32,3>,
    pub rotation: Quaternion32,
    pub scale: NumArray<f32,3>
}

// INITS
impl Transform {
    pub fn default () -> Transform {
        Transform { position: NumArray::zero(), rotation: Quaternion32::zero_rotation(), scale: NumArray::one() }
    }

    pub fn new (position: NumArray<f32, 3>, rotation: Quaternion32, scale: NumArray<f32, 3>) -> Transform {
        Transform { position, rotation, scale }
    }

    pub fn of_position (position: NumArray<f32, 3>) -> Transform {
        Transform { position, rotation: Quaternion32::zero_rotation(), scale: NumArray::one() }
    }

    pub fn of_rotation (rotation: Quaternion32) -> Transform {
        Transform { position: NumArray::zero(), rotation, scale: NumArray::one() }
    }

    pub fn of_scale (scale: NumArray<f32, 3>) -> Transform {
        Transform { position: NumArray::zero(), rotation: Quaternion32::zero_rotation(), scale }
    }

    pub fn of_angles (roll: f32, pitch: f32, yaw: f32) -> Transform {
        Transform { position: NumArray::zero(), rotation: Quaternion32::from_angles(roll, pitch, yaw), scale: NumArray::one() }
    }
}

// METHODS
impl Transform {
    pub fn set_scale (&mut self, value: f32) {
        self.scale = NumArray([value, value, value])
    }

    pub fn rotate (&mut self, roll: f32, pitch: f32, yaw: f32) {
        self.rotation = self.rotation * Quaternion32::from_angles(roll, pitch, yaw);
        self.rotation = self.rotation.unit();
    }

    pub fn position_matrix (&self) -> Matrix4<f32> {
        Matrix4::new([
            NumArray([1., 0., 0., self.position.x()]),
            NumArray([0., 1., 0., self.position.y()]),
            NumArray([0., 0., 1., self.position.z()]),
            NumArray([0., 0., 0., 1.]),
        ])
    }

    pub fn scale_matrix (&self) -> Matrix4<f32> {
        Matrix4::new([
            NumArray([self.scale.x(), 0., 0., 0.]),
            NumArray([0., self.scale.y(), 0., 0.]),
            NumArray([0., 0., self.scale.z(), 0.]),
            NumArray([0., 0., 0., 1.]),
        ])
    }

    pub fn matrix (&self) -> Matrix4<f32> {
        self.position_matrix() * self.rotation.rot_matrix4() * self.scale_matrix()
    }
}