use crate::math::{array_ext::NumArray, quaternion::Quaternion32};

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
    fn rotate (&mut self, roll: f32, pitch: f32, yaw: f32) {
        self.rotation = self.rotation * Quaternion32::from_angles(roll, pitch, yaw);
        self.rotation = self.rotation.unit();
    }
}