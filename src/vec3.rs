//Vector 3 Implementation
//Ewan Burnett (EwanBurnettSK@Outlook.com)

//Stores a 3-component float32 Vector. 
pub struct Vec3{
    pub x: f32, 
    pub y: f32, 
    pub z: f32,
}


impl Vec3{
    pub fn new(_x: f32, _y: f32, _z: f32) -> Vec3 {
        Vec3{x: _x ,y: _y, z: _z}   //Construct a new Vector3 instance. 
    }


    pub fn length_squared(v : &Vec3) -> f32{
        return (v.x * v.x) + (v.y * v.y) + (v.z * v.z);
    }

    /*
    pub fn length(v: &Vec3) -> f32{
        return sqrt(Vec3::length_squared(&v));
    }
   */

    pub fn dot(a: &Vec3, b: &Vec3) -> f32{
        return (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
    }

    pub fn cross(u: &Vec3, v: &Vec3) {  //-> Vec3
        /*
        x : u.y * v.z - u.z * v.y, 
        y : u.x * v.z - u.z * v.x,
        z : u.x * v.y - u.y * v.z,
        */
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
    fn vec3_length_squared(){
        assert_eq!(true, true);
    }


    #[test]
    fn vec3_dot_vec3(){
        assert_eq!(true, true);
    }
    

}