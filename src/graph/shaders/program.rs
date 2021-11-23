use crate::math::matrix::{Matrix2, Matrix3, Matrix4};

use super::{shader::{FragmentShader, VertexShader}};

pub trait Program {
    type Vertex: VertexShader;
    type Fragment: FragmentShader;
    type Uniform: Uniform;

    fn get_vertex (&self) -> &Self::Vertex;
    fn get_fragment (&self) -> &Self::Fragment;
    fn validate (&self);

    fn get_uniforms (&self) -> &[Self::Uniform];
    fn set_bool (&self, key: &Self::Uniform, value: bool); 
    fn set_int (&self, key: &Self::Uniform, value: i32);
    fn set_uint (&self, key: &Self::Uniform, value: u32);
    fn set_float (&self, key: &Self::Uniform, value: f32);
    fn set_double (&self, key: &Self::Uniform, value: f64);

    fn set_bools (&self, key: &Self::Uniform, value: &[bool]);
    fn set_ints (&self, key: &Self::Uniform, value: &[i32]);
    fn set_uints (&self, key: &Self::Uniform, value: &[u32]);
    fn set_floats (&self, key: &Self::Uniform, value: &[f32]);
    fn set_doubles (&self, key: &Self::Uniform, value: &[f64]);

    fn set_float_mat2 (&self, key: &Self::Uniform, value: Matrix2<f32>);
    fn set_float_mat3 (&self, key: &Self::Uniform, value: Matrix3<f32>);
    fn set_float_mat4 (&self, key: &Self::Uniform, value: Matrix4<f32>);

    fn set_double_mat2 (&self, key: &Self::Uniform, value: Matrix2<f64>);
    fn set_double_mat3 (&self, key: &Self::Uniform, value: Matrix3<f64>);
    fn set_double_mat4 (&self, key: &Self::Uniform, value: Matrix4<f64>);

    fn get_uniform (&self, name: &str) -> Option<&Self::Uniform> {
        self.get_uniforms().iter().find(|x| x.get_name() == name)
    }

    fn set_bool_by_name (&self, key: &str, value: bool) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_bool(x, value),
            None => return false
        }

        return true
    }

    fn set_bools_by_name (&self, key: &str, value: &[bool]) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_bools(x, value),
            None => return false
        }

        return true
    }

    fn set_int_by_name (&self, key: &str, value: i32) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_int(x, value),
            None => return false
        }

        return true
    }

    fn set_ints_by_name (&self, key: &str, value: &[i32]) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_ints(x, value),
            None => return false
        }

        return true
    }

    fn set_uint_by_name (&self, key: &str, value: u32) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_uint(x, value),
            None => return false
        }

        return true
    }

    fn set_uints_by_name (&self, key: &str, value: &[u32]) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_uints(x, value),
            None => return false
        }

        return true
    }

    fn set_float_by_name (&self, key: &str, value: f32) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float(x, value),
            None => return false
        }

        return true
    }

    fn set_floats_by_name (&self, key: &str, value: &[f32]) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_floats(x, value),
            None => return false
        }

        return true
    }

    fn set_double_by_name (&self, key: &str, value: f64) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double(x, value),
            None => return false
        }

        return true
    }

    fn set_doubles_by_name (&self, key: &str, value: &[f64]) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_doubles(x, value),
            None => return false
        }

        return true
    }

    fn set_float_mat2_by_name (&self, key: &str, value : Matrix2<f32>) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_mat2(x, value),
            None => return false
        }

        return true
    }

    fn set_float_mat3_by_name (&self, key: &str, value: Matrix3<f32>) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_mat3(x, value),
            None => return false
        }

        return true
    }

    fn set_float_mat4_by_name (&self, key: &str, value: Matrix4<f32>) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_mat4(x, value),
            None => return false
        }

        return true
    }

    fn set_double_mat2_by_name (&self, key: &str, value: Matrix2<f64>) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_mat2(x, value),
            None => return false
        }

        return true
    }

    fn set_double_mat3_by_name (&self, key: &str, value: Matrix3<f64>) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_mat3(x, value),
            None => return false
        }

        return true
    }

    fn set_double_mat4_by_name (&self, key: &str, value: Matrix4<f64>) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_mat4(x, value),
            None => return false
        }

        return true
    }

    fn bind (&self);
    fn unbind (&self);
}

pub trait Uniform {
    fn get_name (&self) -> &str;
}