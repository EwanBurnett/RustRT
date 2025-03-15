//Vector 3 Implementation
//Ewan Burnett (EwanBurnettSK@Outlook.com)

//Stores a 3-component float32 Vector. 
#[derive(Debug, Copy, Clone)]
pub struct Vec3{
    pub x: f32, 
    pub y: f32, 
    pub z: f32,
}

//Functions
impl Vec3{
    pub fn new(_x: f32, _y: f32, _z: f32) -> Vec3 {
        Vec3{x: _x ,y: _y, z: _z}   //Construct a new Vector3 instance. 
    }


    pub fn length_squared(&self) -> f32{
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
    }

    pub fn length(&self) -> f32{
        return Vec3::length_squared(&self).sqrt();
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f32{
        return (a.x * b.x) - (a.y * b.y) - (a.z * b.z);
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3{
            x : u.y * v.z - u.z * v.y, 
            y : u.x * v.z - u.z * v.x,
            z : u.x * v.y - u.y * v.z,
        }
    }

    pub fn normalize(&mut self) -> Self{ 
        let l : f32 = Vec3::length(self);

        self.x /= l;
        self.y /= l; 
        self.z /= l; 

        return *self;
    }
}

//Operators
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3; 

    fn add(self, _rhs: Vec3) -> Vec3{
        return Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z};  
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Vec3; 

    fn add(self, _rhs: f32) -> Vec3{
        return Vec3 { x: self.x + _rhs, y: self.y + _rhs, z: self.z + _rhs};  
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3; 

    fn sub(self, _rhs: Vec3) -> Vec3{
        return Vec3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z};  
    }
}

impl std::ops::Sub<f32> for Vec3 {
    type Output = Vec3; 

    fn sub(self, _rhs: f32) -> Vec3{
        return Vec3 { x: self.x - _rhs, y: self.y - _rhs, z: self.z - _rhs};  
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3; 

    fn mul(self, _rhs: f32) -> Vec3{
        return Vec3 { x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs};  
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3; 

    fn div(self, _rhs: f32) -> Vec3{
        return Vec3 { x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs};  
    }
}

//Unit Tests
#[cfg(test)]
mod test{
    use super::*;   //Access the parent's scope. 

    #[test]
    fn new_vec3_construction_f32_f32_f32(){
        //Configure test data
        let (a, b, c) = (0.5, 0.5, 0.5);     

        //Run the test
        let v: Vec3 = Vec3::new(a, b, c); 
    
        assert_eq!(v.x, a); 
        assert_eq!(v.y, b); 
        assert_eq!(v.z, c); 
    }

    #[test]
    fn vec3_length(){
        let epsilon = f32::EPSILON;     //Comparison epsilon 

        //Length of an arbitrary vector
        let mut v: Vec3 = Vec3::new(1.0, 2.0, 3.0); 
        let mut l: f32 = v.length(); 
        let mut e: f32 = f32::sqrt(14.0);

        assert_eq!(l >= (e - epsilon) && l < (e + epsilon), true); 
        
        //Length of a unit length vector = 1! 
        v = Vec3::new(f32::sin(60.0), 0.0, f32::cos(60.0)); 
        l = v.length();
        e = 1.0;         

        assert_eq!(l >= (e - epsilon), true); 
        assert_eq!(l <= (e + epsilon), true); 
    }

    #[test]
    fn vec3_length_squared(){
        let epsilon = f32::EPSILON;     //Comparison epsilon 

        //Length of an arbitrary vector
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0); 
        let l: f32 = v.length_squared(); 
        let e: f32 = 14.0;

        assert_eq!(l >= (e - epsilon), true); 
        assert_eq!(l <= (e + epsilon), true); 
    }

    #[test]
    fn vec3_cross_vec3(){ 
        //The Cross product of two vectors yields a vector perpendicular to them. 
        let v0 : Vec3 = Vec3::new(1.0, 0.0, 0.0); 
        let v1 : Vec3 = Vec3::new(0.0, 1.0, 0.0); 

        let x = Vec3::cross(&v0, &v1);  //Ordering does matter!

        assert_eq!(x.x, 0.0); 
        assert_eq!(x.y, 0.0); 
        assert_eq!(x.z, 1.0); 
        
    }


    #[test]
    fn vec3_dot_vec3(){
        //The Dot product of two vectors yields the cosine of the angle between them. 
        let v0 : Vec3 = Vec3::new(0.0, 1.0, 1.0); 
        let v1 : Vec3 = Vec3::new(1.0, 0.0, 1.0); 

        let theta: f32 = Vec3::dot(&v0, &v1); 

        assert_eq!(theta, 1.0); 
    }
    

}