use crate::{matrix::{Matf2, Matf3, Matf4, Matd2, Matd3, Matd4}, vector::{EucVecf2, EucVecf3, EucVecf4, EucVecd2, EucVecd3, EucVecd4}};
use super::{shader::{FragmentShader, VertexShader}, UniformValue};

pub trait Program: Sized {
    type Error;
    type Vertex: VertexShader;
    type Fragment: FragmentShader;
    type Uniform: Uniform;

    fn get_vertex (&self) -> &Self::Vertex;
    fn get_fragment (&self) -> &Self::Fragment;
    fn validate (&self) -> Result<(), Self::Error>;

    fn get_uniforms (&self) -> &[Self::Uniform];

    fn set_bool (&mut self, key: &Self::Uniform, value: bool); 
    fn set_int (&mut self, key: &Self::Uniform, value: i32);
    fn set_uint (&mut self, key: &Self::Uniform, value: u32);
    fn set_float (&mut self, key: &Self::Uniform, value: f32);
    fn set_double (&mut self, key: &Self::Uniform, value: f64);

    fn set_bools (&mut self, key: &Self::Uniform, value: &[bool]);
    fn set_ints (&mut self, key: &Self::Uniform, value: &[i32]);
    fn set_uints (&mut self, key: &Self::Uniform, value: &[u32]);
    fn set_floats (&mut self, key: &Self::Uniform, value: &[f32]);
    fn set_doubles (&mut self, key: &Self::Uniform, value: &[f64]);

    fn set_float_vec2 (&mut self, key: &Self::Uniform, value: &EucVecf2);
    fn set_float_vec3 (&mut self, key: &Self::Uniform, value: &EucVecf3);
    fn set_float_vec4 (&mut self, key: &Self::Uniform, value: &EucVecf4);

    fn set_float_mat2 (&mut self, key: &Self::Uniform, value: &Matf2);
    fn set_float_mat3 (&mut self, key: &Self::Uniform, value: &Matf3);
    fn set_float_mat4 (&mut self, key: &Self::Uniform, value: &Matf4);

    fn set_double_vec2 (&mut self, key: &Self::Uniform, value: &EucVecd2);
    fn set_double_vec3 (&mut self, key: &Self::Uniform, value: &EucVecd3);
    fn set_double_vec4 (&mut self, key: &Self::Uniform, value: &EucVecd4);

    fn set_double_mat2 (&mut self, key: &Self::Uniform, value: &Matd2);
    fn set_double_mat3 (&mut self, key: &Self::Uniform, value: &Matd3);
    fn set_double_mat4 (&mut self, key: &Self::Uniform, value: &Matd4);

    fn get_uniform (&self, name: &str) -> Option<&Self::Uniform> {
        self.get_uniforms().iter().find(|x| x.get_name() == name)
    }

    fn get_uniform_clone (&self, name: &str) -> Option<Self::Uniform> where Self::Uniform: Clone {
        self.get_uniforms().iter().find(|x| x.get_name() == name).map(|x| x.clone())
    }

    fn set_uniform <T: UniformValue> (&mut self, key: &str, value: &T) -> bool where Self::Uniform: Clone {
        value.set_to_program(self, key)
    }
}

pub trait Uniform: Sized + Clone {
    fn get_name (&self) -> &str;
    fn get_child<'a, P: Program<Uniform = Self>> (&self, name: &str, program: &'a P) -> Option<&'a Self> {
        let mut full_name = self.get_name().to_string();
        full_name.push_str(".");
        full_name.push_str(name);

        program.get_uniform(full_name.as_str())
    }
}