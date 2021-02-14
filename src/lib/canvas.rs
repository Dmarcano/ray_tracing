use crate::lib::vec3::{Vec3, Number}; 

use std::fs::{File};
use std::path::Path; 
use std::io::Seek;
use std::io; 

// use std::io::Result; 



/// A structure that contains a 2 Vector representation of a world canvas that is drawn on
pub struct Canvas<T: Number> { 

    pub width : usize,  
    pub height : usize, 
    /// The internal 2-D buffer that contains the vector data of the canvas
    pub buffer : Vec<Vec<Vec3<T>>>
}   


impl<T: Number>  Canvas<T> {

    pub fn new(width : usize, height : usize ) -> Canvas<T> { 
        let mut buffer : Vec<Vec<Vec3<T>>> = Vec::with_capacity(width); 

        for row_idx in 0..height { 
            let mut canvas_col : Vec<Vec3<T>> = Vec::with_capacity(height);
            
            for col_idx in 0..width { 
                let empty_vec = Vec3::<T>::zero(); 
                canvas_col[col_idx] = empty_vec; 
            }
            buffer[row_idx] = canvas_col; 
        }

        Canvas { 
            width, 
            height, 
            buffer
        }
    }
}

/// A trait that takes the information of a Canvas and renders it 
pub trait CanvasRender { 

    // for now stick to an u8 canvas
    fn render(&mut self, canvas : &Canvas<u8> ) -> io::Result<()>;
}


/// A simple file format that is easy to create. 
pub struct PpmFile { 
    inner : std::fs::File
}

impl PpmFile { 
    pub fn create<P: AsRef<Path>>(path : P)  -> io::Result<PpmFile> { 

        let inner = File::create(path)?; 
        Ok(PpmFile { 
            inner : inner
        })
    }
}

impl CanvasRender for PpmFile { 

    fn render(&mut self, canvas: &Canvas<u8>)  -> io::Result<()> { 

        self.inner.seek(std::io::SeekFrom::Start(0))?; 
        self.inner.set_len(0)?;
        
        write!(&mut self.inner, "P3\n{} {}\n255\n", canvas.width, canvas.height);

        for row in canvas.buffer.iter() { 
            for color in row.iter() { 
                write!(&mut self.inner, "{} {} {}\n", color.x, color.y, color.z)?;
            }
        };
        Ok(())
    }
}
