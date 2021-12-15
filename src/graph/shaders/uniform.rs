use crate::matrix::{Matf2, Matf3, Matf4, Matd2, Matd3, Matd4};
use super::program::{Program};

// UNIFORM VALUES
pub trait UniformValue {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool where Self: Sized;
    fn set_to_program_by_name<P: Program> (&self, program: &mut P, key: &str) -> bool where Self: Sized {
        program.set_uniform_by_name(key, self)
    }
}

// NATIVES
impl UniformValue for bool {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_bool(key, self.clone());
        true
    }
}

impl UniformValue for &[bool] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_bools(key, self);
        true
    }
}

impl UniformValue for Vec<bool> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_bools(key, self.as_slice());
        true
    }
}

impl UniformValue for i32 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_int(key, self.clone());
        true
    }
}

impl UniformValue for &[i32] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_ints(key, self);
        true
    }
}

impl UniformValue for Vec<i32> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_ints(key, self.as_slice());
        true
    }
}

impl UniformValue for u32 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_uint(key, self.clone());
        true
    }
}

impl UniformValue for &[u32] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_uints(key, self);
        true
    }
}

impl UniformValue for Vec<u32> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_uints(key, self.as_slice());
        true
    }
}

impl UniformValue for f32 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_float(key, self.clone());
        true
    }
}

impl UniformValue for &[f32] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_floats(key, self);
        true
    }
}

impl UniformValue for Vec<f32> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_floats(key, self.as_slice());
        true
    }
}

impl UniformValue for f64 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_double(key, self.clone());
        true
    }
}

impl UniformValue for &[f64] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_doubles(key, self);
        true
    }
}

impl UniformValue for Vec<f64> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_doubles(key, self.as_slice());
        true
    }
}

impl UniformValue for Matf2 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_float_mat2(key, self);
        true
    }
}

impl UniformValue for Matf3 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_float_mat3(key, self);
        true
    }
}

impl UniformValue for Matf4 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_float_mat4(key, self);
        true
    }
}

impl UniformValue for Matd2 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_double_mat2(key, self);
        true
    }
}

impl UniformValue for Matd3 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_double_mat3(key, self);
        true
    }
}

impl UniformValue for Matd4 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &P::Uniform) -> bool {
        program.set_double_mat4(key, self);
        true
    }
}