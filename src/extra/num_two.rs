use num::{BigInt, BigUint, Num, One, bigint::Sign};

pub trait NumericTwo: Num {
    fn two () -> Self;
}

impl NumericTwo for i8 {
    fn two () -> Self {
        2
    }
}

impl NumericTwo for u8 {
    fn two () -> Self {
        2
    }
}

impl NumericTwo for i16 {
    fn two () -> Self {
        2
    }
}

impl NumericTwo for u16 {
    fn two () -> Self {
        2
    }
}

impl NumericTwo for i32 {
    fn two () -> Self {
        2
    }
}

impl NumericTwo for u32 {
    fn two () -> Self {
        2
    }
}

impl NumericTwo for i64 {
    fn two () -> Self {
        2
    }
}

impl NumericTwo for u64 {
    fn two () -> Self {
        2
    }
}

impl NumericTwo for f32 {
    fn two () -> Self {
        2.
    }
}

impl NumericTwo for f64 {
    fn two () -> Self {
        2.
    }
}