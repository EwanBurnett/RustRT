
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct ColourRGBA{
    pub r: u8, 
    pub g: u8, 
    pub b: u8, 
    pub a: u8, 
}

//TODO: Colour Clamping + Colour space conversions!
impl ColourRGBA{
    pub fn new(_r: &u8, _g: &u8, _b: &u8, _a: &u8) -> ColourRGBA{
        ColourRGBA{r: *_r, g: *_g, b: *_b, a: *_a} 
    }

    pub fn new_f32(_r : &f32, _g: &f32, _b: &f32, _a: &f32) -> ColourRGBA{
        ColourRGBA{r: (*_r * 255.99) as u8, g: (*_g * 255.99) as u8, b: (*_b * 255.99) as u8, a: (*_a * 255.99) as u8}
    }

    pub fn new_vec3(vec: &Vec3) -> ColourRGBA{
        let alpha : f32 = 1.0; //Default alpha to 1!
        return ColourRGBA::new_f32(&vec.x, &vec.y, &vec.z, &alpha);
    }
}

impl std::ops::Add<ColourRGBA> for ColourRGBA{
    type Output = ColourRGBA;

    fn add(self, _rhs: ColourRGBA) -> ColourRGBA{
       ColourRGBA{
        r: self.r + _rhs.r,
        g: self.g + _rhs.g,
        b: self.b + _rhs.b,
        a: self.a + _rhs.a,
        } 
    }
}


impl std::ops::Sub<ColourRGBA> for ColourRGBA{
    type Output = ColourRGBA;

    fn sub(self, _rhs: ColourRGBA) -> ColourRGBA{
       ColourRGBA{
        r: self.r - _rhs.r,
        g: self.g - _rhs.g,
        b: self.b - _rhs.b,
        a: self.a - _rhs.a,
        } 
    }
}