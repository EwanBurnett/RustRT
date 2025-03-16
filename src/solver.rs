
pub fn solve_quadratic(a: &f32, b: &f32, c: &f32, t_min: &mut f32, t_max: &mut f32) -> bool {
    
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