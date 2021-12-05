use crate::math::array_ext::NumArray;

/// Trait to streamline reading mouse inputs across various implementation types
pub trait MouseListener {
    /// Relative position of the mouse in a (-1, 1) range
    fn relative_position (&self) -> NumArray<f32, 2>;

    /// Absolute position of the mouse from (0, 0) to (width, height)
    fn absolute_position (&self, size: NumArray<u32, 2>) -> NumArray<u32, 2> {
        let rel = self.relative_position();
        let x = rel.x() * (size[0] as f32);
        let y = rel.y() * (size[1] as f32);

        NumArray([x.round() as u32, y.round() as u32])
    }
}