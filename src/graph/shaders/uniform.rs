use crate::matrix::{Matf2, Matf3, Matf4, Matd2, Matd3, Matd4};
use super::program::{Program};

pub fn subkey (parent: &str, child: &str) -> String {
    let mut name = parent.to_string();
    name.push('.');
    name.push_str(child);

    name
}

// UNIFORM VALUES
pub trait UniformValue {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool where Self: Sized;
}

// NATIVES
impl UniformValue for bool {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_bool(&x.clone(), self.clone())
        }

        true
    }
}

impl UniformValue for &[bool] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_bools(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Vec<bool> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_bools(&x.clone(), self.as_slice())
        }

        true
    }
}

impl UniformValue for i32 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_int(&x.clone(), self.clone())
        }

        true
    }
}

impl UniformValue for &[i32] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_ints(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Vec<i32> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_ints(&x.clone(), self.as_slice())
        }

        true
    }
}

impl UniformValue for u32 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_uint(&x.clone(), self.clone())
        }

        true
    }
}

impl UniformValue for &[u32] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_uints(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Vec<u32> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_uints(&x.clone(), self.as_slice())
        }

        true
    }
}

impl UniformValue for f32 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_float(&x.clone(), self.clone())
        }

        true
    }
}

impl UniformValue for &[f32] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_floats(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Vec<f32> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_floats(&x.clone(), self.as_slice())
        }

        true
    }
}

impl UniformValue for f64 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_double(&x.clone(), self.clone())
        }

        true
    }
}

impl UniformValue for &[f64] {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_doubles(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Vec<f64> {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_doubles(&x.clone(), self.as_slice())
        }

        true
    }
}

impl UniformValue for Matf2 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_float_mat2(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Matf3 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_float_mat3(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Matf4 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_float_mat4(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Matd2 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_double_mat2(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Matd3 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_double_mat3(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for Matd4 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_double_mat4(&x.clone(), self)
        }

        true
    }
}