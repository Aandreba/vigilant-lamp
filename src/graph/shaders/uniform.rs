use crate::math::matrix::{Matrix2, Matrix3, Matrix4};

use super::program::{Program};

// UNIFORM VALUES
pub trait UniformValue {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform);
    fn set_to_program_by_name<P: Program> (self, program: &P, key: &str) -> bool where Self: Sized {
        let uniform = program.get_uniform(key);
        match uniform {
            Some(x) => self.set_to_program(program, x),
            None => return false
        }

        true
    }

}

// NATIVES
impl UniformValue for bool {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_bool(key, self)
    }
}

impl<const S: usize> UniformValue for [bool;S] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_bools(key, &self)
    }
}

impl UniformValue for i32 {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_int(key, self)
    }
}

impl<const S: usize> UniformValue for [i32;S] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_ints(key, &self)
    }
}

impl UniformValue for u32 {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_uint(key, self)
    }
}

impl<const S: usize> UniformValue for [u32;S] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_uints(key, &self)
    }
}

impl UniformValue for f32 {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_float(key, self)
    }
}

impl<const S: usize> UniformValue for [f32;S] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_floats(key, &self)
    }
}

impl UniformValue for Matrix2<f32> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_float_mat2(key, self)
    }
}

impl UniformValue for Matrix3<f32> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_float_mat3(key, self)
    }
}

impl UniformValue for Matrix4<f32> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_float_mat4(key, self)
    }
}

impl UniformValue for f64 {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_double(key, self)
    }
}

impl<const S: usize> UniformValue for [f64;S] {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_doubles(key, &self)
    }
}

impl UniformValue for Matrix2<f64> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_double_mat2(key, self)
    }
}

impl UniformValue for Matrix3<f64> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_double_mat3(key, self)
    }
}

impl UniformValue for Matrix4<f64> {
    fn set_to_program<P: Program> (self, program: &P, key: &P::Uniform) {
        program.set_double_mat4(key, self)
    }
}