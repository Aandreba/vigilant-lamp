use crate::math::array_ext::NumArray;

pub trait MouseListener {
    /// Relative position of the mouse in a (-1, 1) range
    fn relative_position (&self) -> NumArray<f32, 2>;

    fn absolute_position (&self, size: NumArray<u32, 2>) -> NumArray<u32, 2> {
        let rel = self.relative_position();
        let x = rel.x() * (size[0] as f32);
        let y = rel.y() * (size[1] as f32);

        NumArray([x.round() as u32, y.round() as u32])
    }
}