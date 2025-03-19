use std::f32::consts::PI;

use RustRT::{vec3::Vec3, vec4::Vec4, matrix4x4::Matrix4x4, renderer::render, camera::Camera, sphere::Sphere};


fn main() {
    println!("Hello, world!");

    //Set up the Camera
    let origin = Vec3::new(0.0, 0.0, 2.0); 
    let dir = Vec3::new(0.0, 0.0, -1.0); 
    let cam = Camera::new(origin, dir);

    //Configure the output image buffer
    let image_width = 1280; 
    let image_height = 720; 

    let mut image_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(image_width, image_height); 

    //Set up a simple scene. 
    let mut scene : Vec<Sphere> = vec![]; 
    scene.push(Sphere::new(0.2, Matrix4x4::translation(&Vec3{x: 1.0, y: 0.0, z: 0.0})));
    scene.push(Sphere::new(0.5, Matrix4x4::rotation_xyz(&Vec3{x: 0.0, y:10.0 * (PI / 180.0), z: 0.0})));
    scene.push(Sphere::new(0.5, Matrix4x4::translation(&Vec3{x: -1.0, y: -0.0, z: 0.0})));


    render(&cam, &mut scene, &mut image_buffer); 

    image_buffer.save("image.png").unwrap(); 
}
