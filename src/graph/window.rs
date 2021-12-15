use std::any::Any;

use crate::Flattern;

pub trait Window {
    fn get_title (&self) -> &str;
    fn get_width (&self) -> u32;
    fn get_height (&self) -> u32;
    
    fn get_size (&self) -> (u32, u32);
    fn get_aspect_ratio (&self) -> f32 {
        let size = self.get_size();
        return (size.0 as f32) / (size.1 as f32);
    }

    fn clear (&mut self);
    fn update (&mut self);

    /// Method that allows to retrieve preperties that are renderer specific, such as scroll position
    fn get_property (&self, key: &str) -> Option<Box<dyn Any>>;
    fn get_property_as<T: 'static> (&self, key: &str) -> Option<Box<T>> {
        let any = self.get_property(key).map(|x| x.downcast::<T>()).flattern(|| "Error");
        match any {
            Err(_) => None,
            Ok(x) => Some(x)
        }
    }

    fn get_property_copy_as<T: 'static + Copy> (&self, key: &str) -> Option<T> {
        let any = self.get_property(key).map(|x| x.downcast::<T>()).flattern(|| "Error");
        match any {
            Err(_) => None,
            Ok(x) => Some(*x.as_ref())
        }
    }
}