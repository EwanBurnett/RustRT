use std::f32::consts::PI;

use RustRT::{vec3::{self, Vec3}, renderer::render, camera::{self, Camera}};


fn main() {
    println!("Hello, world!");

    //DEBUGGING - Vec3 testing 
    /*
    {
        let mut v0 : Vec3 = Vec3::new(100.0, 100.0, 0.0); 
        v0.normalize();
        let mut v1 : Vec3 = Vec3::new(0.0, 100.0, 0.0); 
        v1.normalize(); 
        
        println!("v0 = {}, {}, {}\nv1 = {}, {}, {}", v0.x, v0.y, v0.z, v1.x, v1.y, v1.z);

        let cos_theta = Vec3::dot(&v0, &v1); 

        println!("cos theta = {}, theta = {}", cos_theta, f32::acos(cos_theta) * (180.0 / PI) as f32); //NOTE: acos returns in radians! 

        let sum = v0 - v1; 
        println!("sum = {}, {}, {}", sum.x, sum.y, sum.z);
    }
    */


    //Set up the Camera
    let origin = Vec3::new(0.0, 0.0, 2.0); 
    let dir = Vec3::new(0.0, 0.0, -1.0); 
    let cam = RustRT::camera::Camera::new(origin, dir);

 
    //Configure the output image buffer
    let image_width = 512; 
    let image_height = 256; 

    let mut image_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(image_width, image_height); 

    render(&cam, &mut image_buffer); 

    image_buffer.save("image.png").unwrap(); 
}
