use crate::{matrix::{Matf2, Matf3, Matf4, Matd2, Matd3, Matd4}, vector::{EucVecf2, EucVecf3, EucVecf4, EucVecd2, EucVecd3, EucVecd4}};
use super::{shader::{FragmentShader, VertexShader}};

pub trait Program {
    type Error;
    type Vertex: VertexShader;
    type Fragment: FragmentShader;
    type Uniform: Uniform;

    fn get_vertex (&self) -> &Self::Vertex;
    fn get_fragment (&self) -> &Self::Fragment;
    fn validate (&self) -> Result<(), Self::Error>;

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

    fn set_float_vec2 (&self, key: &Self::Uniform, value: &EucVecf2);
    fn set_float_vec3 (&self, key: &Self::Uniform, value: &EucVecf3);
    fn set_float_vec4 (&self, key: &Self::Uniform, value: &EucVecf4);

    fn set_float_mat2 (&self, key: &Self::Uniform, value: &Matf2);
    fn set_float_mat3 (&self, key: &Self::Uniform, value: &Matf3);
    fn set_float_mat4 (&self, key: &Self::Uniform, value: &Matf4);

    fn set_double_vec2 (&self, key: &Self::Uniform, value: &EucVecd2);
    fn set_double_vec3 (&self, key: &Self::Uniform, value: &EucVecd3);
    fn set_double_vec4 (&self, key: &Self::Uniform, value: &EucVecd4);

    fn set_double_mat2 (&self, key: &Self::Uniform, value: &Matd2);
    fn set_double_mat3 (&self, key: &Self::Uniform, value: &Matd3);
    fn set_double_mat4 (&self, key: &Self::Uniform, value: &Matd4);

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

    fn set_float_vec2_by_name (&self, key: &str, value: &EucVecf2) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_vec2(x, value),
            None => return false
        }

        return true
    }

    fn set_float_vec3_by_name (&self, key: &str, value: &EucVecf3) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_vec3(x, value),
            None => return false
        }

        return true
    }

    fn set_float_vec4_by_name (&self, key: &str, value: &EucVecf4) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_vec4(x, value),
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

    fn set_double_vec2_by_name (&self, key: &str, value: &EucVecd2) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_vec2(x, value),
            None => return false
        }

        return true
    }

    fn set_double_vec3_by_name (&self, key: &str, value: &EucVecd3) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_vec3(x, value),
            None => return false
        }

        return true
    }

    fn set_double_vec4_by_name (&self, key: &str, value: &EucVecd4) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_vec4(x, value),
            None => return false
        }

        return true
    }

    fn set_float_mat2_by_name (&self, key: &str, value : &Matf2) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_mat2(x, value),
            None => return false
        }

        return true
    }

    fn set_float_mat3_by_name (&self, key: &str, value: &Matf3) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_mat3(x, value),
            None => return false
        }

        return true
    }

    fn set_float_mat4_by_name (&self, key: &str, value: &Matf4) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_float_mat4(x, value),
            None => return false
        }

        return true
    }

    fn set_double_mat2_by_name (&self, key: &str, value: &Matd2) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_mat2(x, value),
            None => return false
        }

        return true
    }

    fn set_double_mat3_by_name (&self, key: &str, value: &Matd3) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_mat3(x, value),
            None => return false
        }

        return true
    }

    fn set_double_mat4_by_name (&self, key: &str, value: &Matd4) -> bool {
        let id = self.get_uniform(key);
        match id {
            Some(x) => self.set_double_mat4(x, value),
            None => return false
        }

        return true
    }
}

pub trait Uniform: Sized {
    fn get_name (&self) -> &str;
    fn get_child<'a, P: Program<Uniform = Self>> (&self, name: &str, program: &'a P) -> Option<&'a Self> {
        let mut full_name = self.get_name().to_string();
        full_name.push_str(".");
        full_name.push_str(name);

        program.get_uniform(full_name.as_str())
    }
}