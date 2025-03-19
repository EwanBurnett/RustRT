//Vector 4 Implementation
//Ewan Burnett (EwanBurnettSK@Outlook.com)

use std::{ops::Index, ptr::null}; 

//Stores a 4-component float42 Vector. 
#[derive(Debug, Copy, Clone)]
pub struct Vec4{
    pub x: f32, 
    pub y: f32, 
    pub z: f32,
    pub w: f32,
}

//Functions
impl Vec4{
    pub fn new(_x: f32, _y: f32, _z: f32, _w: f32) -> Vec4 {
        Vec4{x: _x ,y: _y, z: _z, w: _w}   //Construct a new Vector4 instance. 
    }


    pub fn length_squared(&self) -> f32{
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w);
    }

    pub fn length(&self) -> f32{
        return Vec4::length_squared(&self).sqrt();
    }

    pub fn dot(a: &Vec4, b: &Vec4) -> f32{
        return (a.x * b.x) + (a.y * b.y) + (a.z * b.z) + (a.w * b.w);
    }


    pub fn normalize(&mut self) -> Self{ 
        let l : f32 = Vec4::length(self);

        self.x /= l;
        self.y /= l; 
        self.z /= l; 
        self.w /= l; 

        return *self;
    }
}

//Operators
impl std::ops::Add<Vec4> for Vec4 {
    type Output = Vec4; 

    fn add(self, _rhs: Vec4) -> Vec4{
        return Vec4 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z, w: self.w + _rhs.w};  
    }
}

impl std::ops::Add<f32> for Vec4 {
    type Output = Vec4; 

    fn add(self, _rhs: f32) -> Vec4{
        return Vec4 { x: self.x + _rhs, y: self.y + _rhs, z: self.z + _rhs, w: self.w + _rhs};  
    }
}

impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Vec4; 

    fn sub(self, _rhs: Vec4) -> Vec4{
        return Vec4 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z, w: self.w - _rhs.w};  
    }
}

impl std::ops::Sub<f32> for Vec4 {
    type Output = Vec4; 

    fn sub(self, _rhs: f32) -> Vec4{
        return Vec4 { x: self.x - _rhs, y: self.y - _rhs, z: self.z - _rhs, w: self.w - _rhs};  
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Vec4; 

    fn mul(self, _rhs: f32) -> Vec4{
        return Vec4 { x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs, w: self.w * _rhs};  
    }
}

impl std::ops::Div<f32> for Vec4 {
    type Output = Vec4; 

    fn div(self, _rhs: f32) -> Vec4{
        return Vec4 { x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs, w: self.w / _rhs};  
    }
}

impl std::ops::Index<usize> for Vec4{
    type Output = f32;

    fn index(&self, idx: usize) -> &f32{
        return match (idx % 4){
            0 => &self.x, 
            1 => &self.y, 
            2 => &self.z, 
            3 => &self.w, 
            _ => unreachable!() 
        }
    }
}
impl std::ops::IndexMut<usize> for Vec4{
    //type Output = f32;

    fn index_mut(&mut self, idx: usize) -> &mut f32{
        return match (idx){
            0 => &mut self.x, 
            1 => &mut self.y, 
            2 => &mut self.z, 
            3 => &mut self.w, 
            _ => unreachable!()
        }
    }
}

/*
//Unit Tests
#[cfg(test)]
mod test{
    use super::*;   //Access the parent's scope. 
    
    #[test]
    fn new_vec4_construction_f32_f32_f32(){
        //Configure test data
        let (a, b, c) = (0.5, 0.5, 0.5);     
        
        //Run the test
        let v: Vec4 = Vec4::new(a, b, c); 
        
        assert_eq!(v.x, a); 
        assert_eq!(v.y, b); 
        assert_eq!(v.z, c); 
    }
    
    #[test]
    fn vec4_length(){
        let epsilon = f32::EPSILON;     //Comparison epsilon 
        
        //Length of an arbitrary vector
        let mut v: Vec4 = Vec4::new(1.0, 2.0, 4.0); 
        let mut l: f32 = v.length(); 
        let mut e: f32 = f32::sqrt(14.0);
        
        assert_eq!(l >= (e - epsilon) && l < (e + epsilon), true); 
        
        //Length of a unit length vector = 1! 
        v = Vec4::new(f32::sin(60.0), 0.0, f32::cos(60.0)); 
        l = v.length();
        e = 1.0;         
        
        assert_eq!(l >= (e - epsilon), true); 
        assert_eq!(l <= (e + epsilon), true); 
    }
    
    #[test]
    fn vec4_length_squared(){
        let epsilon = f32::EPSILON;     //Comparison epsilon 
        
        //Length of an arbitrary vector
        let v: Vec4 = Vec4::new(1.0, 2.0, 4.0); 
        let l: f32 = v.length_squared(); 
        let e: f32 = 14.0;
        
        assert_eq!(l >= (e - epsilon), true); 
        assert_eq!(l <= (e + epsilon), true); 
    }
    
    #[test]
    fn vec4_cross_vec4(){ 
        //The Cross product of two vectors yields a vector perpendicular to them. 
        let v0 : Vec4 = Vec4::new(1.0, 0.0, 0.0); 
        let v1 : Vec4 = Vec4::new(0.0, 1.0, 0.0); 
        
        let x = Vec4::cross(&v0, &v1);  //Ordering does matter!
        
        assert_eq!(x.x, 0.0); 
        assert_eq!(x.y, 0.0); 
        assert_eq!(x.z, 1.0); 
        
    }
    
    
    #[test]
    fn vec4_dot_vec4(){
        //The Dot product of two vectors yields the cosine of the angle between them. 
        let v0 : Vec4 = Vec4::new(0.0, 1.0, 0.0); 
        let v1 : Vec4 = Vec4::new(1.0, 0.0, 0.0); 
        
        let theta: f32 = Vec4::dot(&v0, &v1); 
        
        assert_eq!(theta, 0.0); 
    }
    
    
}
*/