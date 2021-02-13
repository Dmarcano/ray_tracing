mod lib;

use std::fs::File; 
use std::io::Write;

use lib::vec3::Color;

fn main() {
    let img_width = 255; 
    let img_height = 255; 

    let mut out_file = File::create("public/out.ppm").unwrap(); 

    write!(&mut out_file, "P3\n{} {}\n255\n", img_width, img_height);
    
    for i in (0..img_height).rev() { 
        for j in 0..img_width { 
            let color = Color { 
                x : (i / (img_width-1)) * 255, 
                y : (j / (img_height-1)) * 255,
                z : (0.25 * 255.0) as u8
            }; 
            write!(&mut out_file, "{} {} {}\n", color.x, color.y, color.z).unwrap() ;
        };
        
    };

}
