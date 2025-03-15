
use image::ImageBuffer; 
pub fn render(img_buf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>){
    let width: u32 = img_buf.width(); 
    let height: u32 = img_buf.height(); 

    //Render the image
    for(x, y, pixel) in img_buf.enumerate_pixels_mut()
    {
        let r: f32 = (x as f32) / (width as f32 - 1.0); 
        let g: f32 = (y as f32) / (height as f32 - 1.0); 
        let b: f32 = 0.0; 

        *pixel = image::Rgb([
            (r * 255.999) as u8, 
            (g * 255.999) as u8, 
            (b * 255.999) as u8 
        ]);
    }

}
