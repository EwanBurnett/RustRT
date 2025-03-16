

use crate::vec3::Vec3; 

pub struct Ray{ 
    pub origin: Vec3, 
    pub direction: Vec3,
}

impl Ray{ 
    pub fn new(origin : Vec3, direction : Vec3) -> Ray{
       return Ray{origin, direction}; 
    }

    pub fn at(&self, t: f32) -> Vec3 { //TODO: Replace with RayHit struct  
        return self.origin + (self.direction * t);
    }
}