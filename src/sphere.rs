
use crate::vec3::Vec3; 
use crate::ray::Ray; 
use crate::rayhit::RayHit; 
use crate::solver::solve_quadratic; 

use std::f32::consts::PI;

pub struct Sphere{
    pub position : Vec3, 
    pub radius : f32, 
}

impl Sphere{
   pub fn new(position: Vec3, radius: f32) -> Sphere{
    Sphere{position, radius} 
   } 

    pub fn intersects(&self, ray: &Ray, rayhit: &mut RayHit) -> bool {
        let pos = self.position; 
        let radius= self.radius;
        {
            let to_center = (ray.origin - pos); 

            let s_a = Vec3::dot(&ray.direction, &ray.direction); 
            let s_b = 2.0 * Vec3::dot(&ray.direction, &to_center); 
            let s_c = Vec3::dot(&to_center, &to_center) - (radius * radius); 
    
            let mut t_min: f32 = 0.0; 
            let mut t_max: f32 = 0.0; 
    
            //Solve the Quadratic
            if(!solve_quadratic(&s_a, &s_b, &s_c, &mut t_min, &mut t_max)){
                return false; 
            }
    
            let mut t = t_min; 
            if(t < 0.0){ 
                t = t_max; 
                if(t < 0.0){ 
                    return false;
                }
            }
    
            rayhit.t = t; 
            rayhit.position = ray.at(t); 
            rayhit.normal = (rayhit.position - pos).normalize();
    
            //TODO: Spherical projection helper functions
            let lat = f32::atan2(rayhit.position.z, f32::sqrt((rayhit.position.x * rayhit.position.x) + (rayhit.position.y * rayhit.position.y))); 
            rayhit.uv.x = (f32::atan2(rayhit.position.y, rayhit.position.x) + PI) / (2.0 * PI);      //Use the Mercator Projection.
            rayhit.uv.y = (f32::log10(f32::tan((lat / 2.0) + (PI / 4.0))) + PI) / (2.0 * PI); 
    
            return true;
        }
    }
}