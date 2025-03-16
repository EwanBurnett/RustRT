
use crate::vec3::Vec3; 

pub struct RayHit{ 
    pub position : Vec3, 
    pub normal : Vec3, 
    pub uv : Vec3, 
}

impl RayHit{
    pub fn new() -> RayHit{ 
        RayHit{
            position: Vec3{x: 0.0, y: 0.0, z: 0.0},
            normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
            uv: Vec3{x: 0.0, y: 0.0, z: 0.0},
        }
    }
}