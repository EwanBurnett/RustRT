
use std::f32::consts::PI;

use image::ImageBuffer; 
use crate::vec3::Vec3; 
use crate::ray::Ray;
use crate::camera::Camera; 
use crate::rayhit::RayHit; 
use crate::sphere::Sphere; 


pub fn render(camera : &Camera, img_buf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>){
    let width: u32 = img_buf.width(); 
    let height: u32 = img_buf.height(); 

    let aspect_ratio: f32 = width as f32 / height as f32; 

    
    
    let sphere : Sphere = Sphere::new(Vec3{x: 0.0, y: 0.0, z: 0.0}, 1.0); 

    //Render the image
    for(x, y, pixel) in img_buf.enumerate_pixels_mut()
    {
        /*  //Uncomment to draw Image UVs.
        let r: f32 = (x as f32) / (width as f32 - 1.0); 
        let g: f32 = (y as f32) / (height as f32 - 1.0); 
        let b: f32 = 0.0; 
        */

        //Generate a ray per pixel, taking into account image aspect ratio. 
        
        //Field of View in radians
        let fov_y : f32 = 30.0 * PI / 180.0; //TODO Camera FoV
        let fov_x = 2.0 * f32::atan(f32::tan(fov_y * 0.5) * aspect_ratio); 

        let alpha = 4.0 * f32::tan(fov_x / 2.0) * ((x as f32 - (width as f32 / 2.0) )/ (width as f32 / 2.0));
        let beta = 4.0 * -f32::tan(fov_y / 2.0) * ((y as f32 - (height as f32 / 2.0)) / (height as f32 / 2.0));
        
        let r = camera.gen_ray(alpha, beta); 
        let mut p = Vec3::new(0.0, 0.0, 0.0); // = r.at(1.0); 
        /*
        println!("[{}, {}]\no = {}, {}, {}\nd: {}, {}, {}, at 5.0 = {}, {}, {}",
        x, y, 
        r.origin.x, r.origin.y, r.origin.z, 
        r.direction.x, r.direction.y, r.direction.z, 
        p.x, p.y, p.z
        );
        */

        let mut ray_hit : RayHit = RayHit::new();  
        if(sphere.intersects(&r, &mut ray_hit))
        {
            //println!("ray!");
            p = ray_hit.position; 
            //p = ray_hit.normal; 
            //p = ray_hit.uv; 
        }


        let r: f32 = p.x; 
        let g: f32 = p.y; 
        let b: f32 = p.z; 

        *pixel = image::Rgb([
            (r * 255.999) as u8, 
            (g * 255.999) as u8, 
            (b * 255.999) as u8 
        ]);
    }

}
