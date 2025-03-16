
use std::f32::consts::PI;

use image::ImageBuffer; 
use crate::vec3::Vec3; 
use crate::ray::Ray;
use crate::camera::Camera; 


fn solve_quadratic(a: &f32, b: &f32, c: &f32, t_min: &mut f32, t_max: &mut f32) -> bool {
    
    let discriminant: f32 = (b * b) - (4.0 * a * c); 
    if(discriminant < 0.0){
         return false; 
    }
    else if(discriminant <= 0.01){
        *t_min = -0.5 * b / a;  
        *t_max = *t_min; 
    }

    else{
        let mut min = (-b - f32::sqrt(discriminant)) / (2.0 * a); 
        let mut max = (-b + f32::sqrt(discriminant)) / (2.0 * a); 

        if(min > max){
            let tmp = min; 
            min = max; 
            max = tmp; 
        }

        *t_min = min; 
        *t_max = max; 
    }

    return true; 
}

fn ray_sphere_intersects(ray: &Ray) -> bool{
    //Red sphere of justice!
    let pos = Vec3::new(0.0, 0.0, 0.0); 
    let radius: f32 = 1.0; 
    {
        
        let to_center = (ray.origin - pos); 
        //println!("to_center = {}, {}, {}", to_center.x, to_center.y, to_center.z); 
        let s_a = Vec3::dot(&ray.direction, &ray.direction); 
        let s_b = 2.0 * Vec3::dot(&ray.direction, &to_center); 
        let s_c = Vec3::dot(&to_center, &to_center) - (radius * radius); 

        //println!("a: {}, b: {}, c: {}", s_a, s_b, s_c); 
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

        return true;
    }
}

pub fn render(camera : &Camera, img_buf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>){
    let width: u32 = img_buf.width(); 
    let height: u32 = img_buf.height(); 

    let aspect_ratio: f32 = width as f32 / height as f32; 

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
        let fov_y : f32 = 110.0 * PI / 180.0; //TODO Camera FoV
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

        if(ray_sphere_intersects(&r))
        {
            //println!("ray!");
            p.x = 1.0; 
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
