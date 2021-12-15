pub mod quaternion;
pub mod matrix;
pub mod vector;

// minmax
pub trait Minmax: Ord + Sized {
    fn min_max (self, other: Self) -> (Self, Self) {
        if self >= other {
            return (other, self)
        }

        (self, other)
    }

    fn max_min (self, other: Self) -> (Self, Self) {
        if self >= other {
            return (self, other)
        }

        (other, self)
    }
}

impl<T: Ord + Sized> Minmax for T {}