
use crate::vec3::Vec3; 
use crate::ray::Ray; 

pub struct Camera
{
    //TODO: Replace these attributes with a View Matrix! (4x3)
    //Camera Origin
    pub origin : Vec3,
    
    //Local Camera Basis
    pub right : Vec3,  
    pub up : Vec3, 
    pub forwards : Vec3,  
}

impl Camera{
    pub fn new(origin: Vec3, mut direction: Vec3) -> Camera { 
        let world_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);

        let forwards = direction.normalize(); 

        //Compute the basis based on world up 
        
        let right  = Vec3::cross( &forwards, &world_up).normalize();
        //let right  = Vec3::cross( &world_up, &forwards).normalize();
        let up  = Vec3::cross( &forwards, &right).normalize();

        //Debugging Camera basis / Construction
        println!("right: {}, {}, {}\nup: {}, {}, {}\nforwards: {}, {}, {}\norigin: {}, {}, {}", 
            right.x, right.y, right.z,  
            up.x, up.y, up.z, 
            forwards.x, forwards.y, forwards.z, 
            origin.x, origin.y, origin.z
        );

        return Camera{origin, up, right, forwards}; 
    }

    pub fn gen_ray(&self, dx: f32, dy: f32) -> Ray{ 
        let origin = self.origin;
        let direction = ((self.right * dx) + (self.up * dy) + self.forwards).normalize(); 

        return Ray{origin, direction};
    }
    
}
