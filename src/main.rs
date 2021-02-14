mod lib;

use lib::vec3::Color;
use lib::canvas::{Canvas, CanvasRender, PpmFile};

fn main() {
    let img_width : usize= 256; 
    let img_height : usize = 256; 

    let mut out_file = PpmFile::create("public/out.ppm").unwrap(); 


    let mut canvas = Canvas::<u8>::new(img_width, img_height);
    
    for i in (0..img_height).rev() { 
        for j in 0..img_width { 
            let red = i as f64/ (img_width - 1) as f64; 
            let blue = j as f64 / (img_width - 1) as f64; 
            let green = 0.25 * 255.0;

            let color = Color { 
                x : (red * 255.999) as u8,
                y : (blue *255.999) as u8,
                z : green as u8
            }; 

            canvas.buffer[i][j] = color;

        };
    };

    out_file.render(&canvas).unwrap(); 
}
