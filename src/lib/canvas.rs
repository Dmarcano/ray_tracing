use crate::lib::vec3::Number; 
use crate::lib::vec3::Vec3; 

/// A structure that contains a 2 Vector representation of a world canvas that is drawn on
pub struct Canvas<T: Number> { 

    pub width : usize,  
    pub height : usize, 
    pub canvas : Vec<Vec<Vec3<T>>>
}   


impl<T: Number>  Canvas<T> {

    pub fn new(width : usize, height : usize ) -> Canvas<T> { 
        let mut canvas : Vec<Vec<Vec3<T>>> = Vec::with_capacity(width); 

        for row_idx in 0..height { 
            let mut canvas_col : Vec<Vec3<T>> = Vec::with_capacity(height);
            
            for col_idx in 0..width { 
                let empty_vec = Vec3::<T>::zero(); 
                canvas_col[col_idx] = empty_vec; 
            }
            canvas[row_idx] = canvas_col; 
        }

        Canvas { 
            width, 
            height, 
            canvas
        }
    }
 }