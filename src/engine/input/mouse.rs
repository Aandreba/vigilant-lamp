use crate::vector::{EucVecf2, EucVec2};

/// Trait to streamline reading mouse inputs across various implementation types
pub trait MouseListener {
    fn init () -> Self;
    
    /// Relative position of the mouse in a (-1, 1) range
    fn relative_position (&self) -> EucVecf2;

    /// Absolute position of the mouse from (0, 0) to (width, height)
    fn absolute_position (&self, size: EucVec2<u32>) -> EucVec2<u32> {
        let rel = self.relative_position();
        let x = rel.x * (size.x as f32);
        let y = rel.y * (size.y as f32);

        EucVec2::new(x.round() as u32, y.round() as u32)
    }
}