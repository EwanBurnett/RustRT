
use std::f32::INFINITY;
use std::f32::consts::PI;
use image::ImageBuffer; 
use crate::vec3::Vec3; 
use crate::camera::Camera; 
use crate::ray::Ray; 
use crate::rayhit::RayHit; 
use crate::sphere::Sphere; 
use crate::colour::ColourRGBA; 


pub fn render(camera : &Camera, scene : &Vec<Sphere>, img_buf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>){
    let width: u32 = img_buf.width(); 
    let height: u32 = img_buf.height(); 

    let aspect_ratio: f32 = width as f32 / height as f32; 

    //Render the image
    for(x, y, pixel) in img_buf.enumerate_pixels_mut()
    {
        //Generate a ray per pixel, taking into account image aspect ratio. 
        
        //Field of View in radians
        let fov_y : f32 = 30.0 * PI / 180.0; //TODO Camera FoV
        let fov_x = 2.0 * f32::atan(f32::tan(fov_y * 0.5) * aspect_ratio); 

        let alpha = 4.0 * f32::tan(fov_x / 2.0) * ((x as f32 - (width as f32 / 2.0) )/ (width as f32 / 2.0));
        let beta = 4.0 * -f32::tan(fov_y / 2.0) * ((y as f32 - (height as f32 / 2.0)) / (height as f32 / 2.0));
        
        let r = camera.gen_ray(alpha, beta); 

        let mut colour_vec3 = Vec3::new(0.0, 0.0, 1.0);   //Clear Colour
        
        let mut max_depth = INFINITY; 

        for sphere in scene {
            let mut ray_hit : RayHit = RayHit::new();  
            if(sphere.intersects(&r, &mut ray_hit))
            {
                //Apply some simple lambertian lighting
                if(ray_hit.t < max_depth){
                    max_depth = ray_hit.t; 
                    let light_dir : Vec3 = Vec3::new(0.0, 1.0, 0.0).normalize(); 
                    let light_colour : Vec3 = Vec3::new(0.7, 0.1, 0.1); 
                    let light_intensity : f32 = 1.0; 


                    //Trace a shadow ray
                    //from hit point to light source
                    let mut in_shadow : bool = false; 
                    {
                        let shadow_bias : Vec3 =  ray_hit.normal * 0.01; 
                        let shadow_ray : Ray = Ray::new(ray_hit.position + shadow_bias, light_dir); 
                        let mut shadow_hit : RayHit = RayHit::new(); 

                        for shadow_sphere in scene{ 
                            if(shadow_sphere.intersects(&shadow_ray, &mut shadow_hit)){
                                in_shadow = true;  
                                break;
                            }
                        }
                    }

                    let n_dot_l : f32 = f32::max(Vec3::dot(&light_dir, &ray_hit.normal), 0.0); 

                    let mut shadowf : f32 = 0.0; 
                    if !in_shadow {
                        shadowf = 1.0; 
                    }
                        
                    colour_vec3 = light_colour * light_intensity * n_dot_l * shadowf; 
                }

            }
        }


        let colour_rgba = ColourRGBA::new_vec3(&colour_vec3); 

        *pixel = image::Rgb([
            colour_rgba.r, 
            colour_rgba.g, 
            colour_rgba.b, 
        ]);
    }

}
