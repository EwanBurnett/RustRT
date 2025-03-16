
use crate::ray::Ray; 
use crate::rayhit::RayHit; 

pub trait Primitive{
     fn intersects(&self, ray: &Ray, rayhit: &mut RayHit) -> bool; 
}