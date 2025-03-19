
use std::ops::Index;
use std::process::Output;

use crate::vec3::Vec3; 
use crate::vec4::Vec4;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4{
    pub _arr: [f32; 16],
}

impl Matrix4x4{ 

    pub fn new() -> Matrix4x4{
        let mut out : Matrix4x4 = Matrix4x4{_arr: [0.0; 16]};

        for i in 0..15{
            out._arr[i] = 0.0; 
        }
        //Set the Identity Matrix. 
        out._arr[0] = 1.0; 
        out._arr[5] = 1.0; 
        out._arr[10] = 1.0; 
        out._arr[15] = 1.0; 

        return out; 
    }

    pub fn identity() -> Matrix4x4{
        let mut mat = Matrix4x4::new(); 
        mat._arr[0] = 1.0;
        mat._arr[5] = 1.0; 
        mat._arr[10] = 1.0; 
        mat._arr[15] = 1.0; 

        return mat; 
    }

    pub fn transpose(mat : &Matrix4x4) -> Matrix4x4{ 
        let mut transpose = *mat; 
     //Preserve 0, 5, 10, 15
     transpose._arr[1] = mat._arr[4];
     transpose._arr[2] = mat._arr[8];
     transpose._arr[3] = mat._arr[12];
     transpose._arr[4] = mat._arr[1];
     transpose._arr[6] = mat._arr[9];
     transpose._arr[7] = mat._arr[13];
     transpose._arr[8] = mat._arr[2];
     transpose._arr[9] = mat._arr[6];
     transpose._arr[11] = mat._arr[14];
     transpose._arr[12] = mat._arr[3];
     transpose._arr[13] = mat._arr[7];
     transpose._arr[14] = mat._arr[11];
        

        return transpose; 
    }
    
    pub fn translation(translation : &Vec3) -> Matrix4x4
    {
        let mut mat = Matrix4x4::new(); 

        mat._arr[12] = translation.x; 
        mat._arr[13] = translation.y; 
        mat._arr[14] = translation.z; 

        return mat;
    }

    pub fn scale(scale : &Vec3) -> Matrix4x4
    {
        let mut mat = Matrix4x4::new(); 

        mat._arr[0] = scale.x; 
        mat._arr[5] = scale.y; 
        mat._arr[10] = scale.z; 

        return mat;   

    }

    pub fn rotation_x(x: f32) -> Matrix4x4{ 
        let mut mat = Matrix4x4::new(); 
        mat._arr[5] = f32::cos(x); 
        mat._arr[6] = -f32::sin(x); 
        mat._arr[9] = f32::sin(x); 
        mat._arr[10] = f32::cos(x); 
        return mat;
    }
    
    pub fn rotation_y(y: f32) -> Matrix4x4{ 
        let mut mat = Matrix4x4::new();  
        mat._arr[0] = f32::cos(y); 
        mat._arr[2] = -f32::sin(y); 
        mat._arr[8] = f32::sin(y); 
        mat._arr[10] = f32::cos(y); 
        
        return mat;
    }

    pub fn rotation_z(z: f32) -> Matrix4x4{ 
        let mut mat = Matrix4x4::new(); 

        mat._arr[0] = f32::cos(z); 
        mat._arr[1] = -f32::sin(z); 
        mat._arr[4] = f32::sin(z); 
        mat._arr[5] = f32::cos(z); 
        
        return mat;
    }

    pub fn rotation_xyz(rotation: &Vec3) -> Matrix4x4 
    {
        return Matrix4x4::rotation_x(rotation.x) * (Matrix4x4::rotation_y(rotation.y) * Matrix4x4::rotation_z(rotation.z));
    }

    pub fn inverse(mat: &mut Matrix4x4, inverseExists: &mut bool) -> Matrix4x4{

        let mut inv = Matrix4x4::new(); 

        let mut det: f32; 

        
        inv._arr[0] = mat._arr[5] * mat._arr[10] * mat._arr[15] -
        mat._arr[5] * mat._arr[11] * mat._arr[14] -
        mat._arr[9] * mat._arr[6] * mat._arr[15] +
        mat._arr[9] * mat._arr[7] * mat._arr[14] +
        mat._arr[13] * mat._arr[6] * mat._arr[11] -
        mat._arr[13] * mat._arr[7] * mat._arr[10];

    inv._arr[4] = -mat._arr[4] * mat._arr[10] * mat._arr[15] +
        mat._arr[4] * mat._arr[11] * mat._arr[14] +
        mat._arr[8] * mat._arr[6] * mat._arr[15] -
        mat._arr[8] * mat._arr[7] * mat._arr[14] -
        mat._arr[12] * mat._arr[6] * mat._arr[11] +
        mat._arr[12] * mat._arr[7] * mat._arr[10];

    inv._arr[8] = mat._arr[4] * mat._arr[9] * mat._arr[15] -
        mat._arr[4] * mat._arr[11] * mat._arr[13] -
        mat._arr[8] * mat._arr[5] * mat._arr[15] +
        mat._arr[8] * mat._arr[7] * mat._arr[13] +
        mat._arr[12] * mat._arr[5] * mat._arr[11] -
        mat._arr[12] * mat._arr[7] * mat._arr[9];

    inv._arr[12] = -mat._arr[4] * mat._arr[9] * mat._arr[14] +
        mat._arr[4] * mat._arr[10] * mat._arr[13] +
        mat._arr[8] * mat._arr[5] * mat._arr[14] -
        mat._arr[8] * mat._arr[6] * mat._arr[13] -
        mat._arr[12] * mat._arr[5] * mat._arr[10] +
        mat._arr[12] * mat._arr[6] * mat._arr[9];

    inv._arr[1] = -mat._arr[1] * mat._arr[10] * mat._arr[15] +
        mat._arr[1] * mat._arr[11] * mat._arr[14] +
        mat._arr[9] * mat._arr[2] * mat._arr[15] -
        mat._arr[9] * mat._arr[3] * mat._arr[14] -
        mat._arr[13] * mat._arr[2] * mat._arr[11] +
        mat._arr[13] * mat._arr[3] * mat._arr[10];

    inv._arr[5] = mat._arr[0] * mat._arr[10] * mat._arr[15] -
        mat._arr[0] * mat._arr[11] * mat._arr[14] -
        mat._arr[8] * mat._arr[2] * mat._arr[15] +
        mat._arr[8] * mat._arr[3] * mat._arr[14] +
        mat._arr[12] * mat._arr[2] * mat._arr[11] -
        mat._arr[12] * mat._arr[3] * mat._arr[10];

    inv._arr[9] = -mat._arr[0] * mat._arr[9] * mat._arr[15] +
        mat._arr[0] * mat._arr[11] * mat._arr[13] +
        mat._arr[8] * mat._arr[1] * mat._arr[15] -
        mat._arr[8] * mat._arr[3] * mat._arr[13] -
        mat._arr[12] * mat._arr[1] * mat._arr[11] +
        mat._arr[12] * mat._arr[3] * mat._arr[9];

    inv._arr[13] = mat._arr[0] * mat._arr[9] * mat._arr[14] -
        mat._arr[0] * mat._arr[10] * mat._arr[13] -
        mat._arr[8] * mat._arr[1] * mat._arr[14] +
        mat._arr[8] * mat._arr[2] * mat._arr[13] +
        mat._arr[12] * mat._arr[1] * mat._arr[10] -
        mat._arr[12] * mat._arr[2] * mat._arr[9];

    inv._arr[2] = mat._arr[1] * mat._arr[6] * mat._arr[15] -
        mat._arr[1] * mat._arr[7] * mat._arr[14] -
        mat._arr[5] * mat._arr[2] * mat._arr[15] +
        mat._arr[5] * mat._arr[3] * mat._arr[14] +
        mat._arr[13] * mat._arr[2] * mat._arr[7] -
        mat._arr[13] * mat._arr[3] * mat._arr[6];

    inv._arr[6] = -mat._arr[0] * mat._arr[6] * mat._arr[15] +
        mat._arr[0] * mat._arr[7] * mat._arr[14] +
        mat._arr[4] * mat._arr[2] * mat._arr[15] -
        mat._arr[4] * mat._arr[3] * mat._arr[14] -
        mat._arr[12] * mat._arr[2] * mat._arr[7] +
        mat._arr[12] * mat._arr[3] * mat._arr[6];

    inv._arr[10] = mat._arr[0] * mat._arr[5] * mat._arr[15] -
        mat._arr[0] * mat._arr[7] * mat._arr[13] -
        mat._arr[4] * mat._arr[1] * mat._arr[15] +
        mat._arr[4] * mat._arr[3] * mat._arr[13] +
        mat._arr[12] * mat._arr[1] * mat._arr[7] -
        mat._arr[12] * mat._arr[3] * mat._arr[5];

    inv._arr[14] = -mat._arr[0] * mat._arr[5] * mat._arr[14] +
        mat._arr[0] * mat._arr[6] * mat._arr[13] +
        mat._arr[4] * mat._arr[1] * mat._arr[14] -
        mat._arr[4] * mat._arr[2] * mat._arr[13] -
        mat._arr[12] * mat._arr[1] * mat._arr[6] +
        mat._arr[12] * mat._arr[2] * mat._arr[5];

    inv._arr[3] = -mat._arr[1] * mat._arr[6] * mat._arr[11] +
        mat._arr[1] * mat._arr[7] * mat._arr[10] +
        mat._arr[5] * mat._arr[2] * mat._arr[11] -
        mat._arr[5] * mat._arr[3] * mat._arr[10] -
        mat._arr[9] * mat._arr[2] * mat._arr[7] +
        mat._arr[9] * mat._arr[3] * mat._arr[6];

    inv._arr[7] = mat._arr[0] * mat._arr[6] * mat._arr[11] -
        mat._arr[0] * mat._arr[7] * mat._arr[10] -
        mat._arr[4] * mat._arr[2] * mat._arr[11] +
        mat._arr[4] * mat._arr[3] * mat._arr[10] +
        mat._arr[8] * mat._arr[2] * mat._arr[7] -
        mat._arr[8] * mat._arr[3] * mat._arr[6];

    inv._arr[11] = -mat._arr[0] * mat._arr[5] * mat._arr[11] +
        mat._arr[0] * mat._arr[7] * mat._arr[9] +
        mat._arr[4] * mat._arr[1] * mat._arr[11] -
        mat._arr[4] * mat._arr[3] * mat._arr[9] -
        mat._arr[8] * mat._arr[1] * mat._arr[7] +
        mat._arr[8] * mat._arr[3] * mat._arr[5];

    inv._arr[15] = mat._arr[0] * mat._arr[5] * mat._arr[10] -
        mat._arr[0] * mat._arr[6] * mat._arr[9] -
        mat._arr[4] * mat._arr[1] * mat._arr[10] +
        mat._arr[4] * mat._arr[2] * mat._arr[9] +
        mat._arr[8] * mat._arr[1] * mat._arr[6] -
        mat._arr[8] * mat._arr[2] * mat._arr[5];

    det = mat._arr[0] * inv._arr[0] + mat._arr[1] * inv._arr[4] + mat._arr[2] * inv._arr[8] + mat._arr[3] * inv._arr[12];


    if det == 0.0{
        *inverseExists = false; 
        return Matrix4x4::identity(); 
    }

    *inverseExists = true; 
    det = 1.0 / det; 

    for i in 0..15{
        inv._arr[i] = inv._arr[i] * det;
    }

    return inv;

    }


}

impl std::ops::Mul<Matrix4x4> for Vec4{
    type Output = Vec4;

    fn mul(self, rhs : Matrix4x4) -> Vec4{ 
        let mut out = Vec4::new(0.0, 0.0, 0.0, 0.0); 

        for col in 0..4{

            let col_v = Vec4::new(
                rhs._arr[(0 * 4) + col],   
                rhs._arr[(1 * 4) + col],                
                rhs._arr[(2 * 4) + col], 
                rhs._arr[(3 * 4) + col], 
            ); 

            out[col] = Vec4::dot(&self, &col_v); //Dot rows by columns
        }
        return out; 
    }

}
impl std::ops::Mul<Matrix4x4> for Matrix4x4{
    type Output = Matrix4x4;

    //TODO: FIX ME!
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4{
        let mut mat = Matrix4x4::new(); 

        for row in 0..4{
            let row_v = Vec4::new(
                self._arr[(row * 4) + 0],
                self._arr[(row * 4) + 1],
                self._arr[(row * 4) + 2],
                self._arr[(row * 4) + 3],
            );

            for col in 0..4{

                let col_v = Vec4::new(
                    rhs._arr[(0 * 4) + col],   
                    rhs._arr[(1 * 4) + col],                
                    rhs._arr[(2 * 4) + col], 
                    rhs._arr[(3 * 4) + col], 
                ); 

                //println!("MMULT DOT: Row {}, {}, {} {} dot Col {} {} {} {}", row_v.x, row_v.y, row_v.z, row_v.w, col_v.x, col_v.y, col_v.z, col_v.w); 
                mat._arr[(row * 4) + col] = Vec4::dot(&row_v, &col_v); //Dot rows by columns
            }
        }
        
        
        return mat;
    }
}