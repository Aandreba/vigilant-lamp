use crate::math::matrix::{Matrix2, Matrix3, Matrix4};

use super::program::{Program};

// UNIFORM VALUES
pub trait UniformValue {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool;
    fn set_to_program_by_name<P: Program> (self, program: &P, key: &str) -> bool where Self: Sized {
        let uniform = program.get_uniform(key);
        match uniform {
            Some(x) => return self.set_to_program(program, x),
            None => return false
        }
    }

}

// NATIVES
impl UniformValue for bool {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_bool(key, self);
        true
    }
}

impl UniformValue for &[bool] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_bools(key, self);
        true
    }
}

impl UniformValue for i32 {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_int(key, self);
        true
    }
}

impl UniformValue for &[i32] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_ints(key, self);
        true
    }
}

impl UniformValue for u32 {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_uint(key, self);
        true
    }
}

impl UniformValue for &[u32] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_uints(key, self);
        true
    }
}

impl UniformValue for f32 {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_float(key, self);
        true
    }
}

impl UniformValue for &[f32] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_floats(key, self);
        true
    }
}

impl UniformValue for Matrix2<f32> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_float_mat2(key, self);
        true
    }
}

impl UniformValue for Matrix3<f32> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_float_mat3(key, self);
        true
    }
}

impl UniformValue for Matrix4<f32> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_float_mat4(key, self);
        true
    }
}

impl UniformValue for f64 {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_double(key, self);
        true
    }
}

impl<const S: usize> UniformValue for [f64;S] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_doubles(key, &self);
        true
    }
}

impl UniformValue for Matrix2<f64> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_double_mat2(key, self);
        true
    }
}

impl UniformValue for Matrix3<f64> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_double_mat3(key, self);
        true
    }
}

impl UniformValue for Matrix4<f64> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) -> bool {
        program.set_double_mat4(key, self);
        true
    }
}