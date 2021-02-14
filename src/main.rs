mod lib;

use std::fs::File; 
use std::io::Write;

use lib::vec3::Color;

fn main() {
    let img_width : usize= 256; 
    let img_height : usize = 256; 

    let mut out_file = File::create("public/out.ppm").unwrap(); 

    write!(&mut out_file, "P3\n{} {}\n255\n", img_width, img_height).unwrap();
    
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
            write!(&mut out_file, "{} {} {}\n", color.x, color.y, color.z).unwrap() ;
        };
        
    };

}
