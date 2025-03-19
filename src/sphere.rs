
use crate::vec3::Vec3; 
use crate::vec4::Vec4; 
use crate::ray::Ray; 
use crate::rayhit::RayHit; 
use crate::solver::solve_quadratic; 
use crate::matrix4x4::Matrix4x4;
use std::f32::consts::PI;

pub struct Sphere{
    pub radius : f32, 
    pub transform : Matrix4x4, 
}

impl Sphere{
   pub fn new(radius: f32, transform: Matrix4x4) -> Sphere{
    Sphere{radius, transform} 
   } 

    pub fn intersects(&self, ray: &Ray, rayhit: &mut RayHit) -> bool {
        
        let mut is_invertable : bool = false; 
        let inverse_transform = Matrix4x4::inverse(&self.transform, &mut is_invertable); 
        if !is_invertable{
            return false; 
        }

        //Apply the inverse transform of the ray
        let transformed_origin = Vec4::from_vec3(&ray.origin, 1.0) * inverse_transform;
        let transformed_direction = Vec4::from_vec3(&ray.direction, 0.0) * inverse_transform; 
        let ray = Ray::new(Vec3::from_vec4(&transformed_origin), Vec3::from_vec4(&transformed_direction));


        let pos = Vec3::new(self.transform._arr[12], self.transform._arr[13], self.transform._arr[14]); 
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