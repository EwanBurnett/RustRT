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
    scene.push(Sphere::new(Vec3{x: 0.6, y: 1.0, z: 0.0}, 0.5));
    scene.push(Sphere::new(Vec3{x: 1.0, y: -1.0, z: 0.0}, 0.5));
    scene.push(Sphere::new(Vec3{x: -1.0, y: -1.0, z: 0.0},0.5));
    /*
    for i in 0..10{
        scene.push(Sphere::new(Vec3{x: 0.0, y: f32::cos(i as f32) * 2.0, z: f32::sin((i - 5) as f32)}, 0.5));
    } 
    */

    let a = Matrix4x4::scale(&Vec3{x: 3.0, y: -1.0, z: 0.0}); //Matrix4x4::rotation_z(90.0 * (180.0 / 3.1415926)); 
    let mut b = Matrix4x4::new(); 
    b._arr[5] = 2.0; 

    let mut d = Vec4::new(1.0, 1.0, 1.0, 1.0);
let e = d * a; 


    let c = a * a; 

    for i in 0..4 {
        println!("{}", e[i]);
    }

    render(&cam, &mut scene, &mut image_buffer); 

    image_buffer.save("image.png").unwrap(); 
}
