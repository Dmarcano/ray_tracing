mod lib;

use lib::vec3::Color;
use lib::canvas::{Canvas, CanvasRender, PpmFile};
use lib::ray::Ray; 

fn main() {
    let img_width : usize= 256; 
    let img_height : usize = 256; 
    let mut out_file = PpmFile::create("public/out.ppm").unwrap(); 
    let mut canvas = Canvas::<u8>::new(img_width, img_height);

    draw_gradient(&mut canvas);
    out_file.render(&canvas).unwrap(); 
}

fn ray_color(ray : &Ray) { 
    let unit_vec = ray.direction.unit_vec();
    // let t = 
}

fn draw_gradient(canvas : &mut Canvas<u8>) { 
    let img_height = canvas.height; 
    let img_width = canvas.width; 

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
}