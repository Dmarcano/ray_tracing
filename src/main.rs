mod lib;

use lib::vec3::{Color, Vec3, Point};
use lib::canvas::{Canvas, CanvasRender, PpmFile};
use lib::ray::Ray; 

fn main() {
    let aspect_ration = 16.0/9.0;
    let img_width : usize= 400; 
    let img_height : usize = (img_width as f64 / aspect_ration) as usize; 
    let mut out_file = PpmFile::create("public/out.ppm").unwrap(); 
    let mut canvas = Canvas::<u8>::new(img_width, img_height);

    draw_lerp(&mut canvas);
    out_file.render(&canvas).unwrap(); 
}

fn ray_color(ray : &Ray) ->Vec3<f64>{ 
    let unit_vec = ray.direction.unit_vec();
    let t = (unit_vec.y + 1.0)*0.5;
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
}

fn draw_lerp(canvas: &mut Canvas<u8>) { 
    let width = canvas.width; 
    let height = canvas.height; 

    // setup camera 
    let viewport_height = 2.0; 
    let viewport_width = 16.0/9.0 * viewport_height; 
    let focal_len = 1.0; 
    
    let origin = Point::zero();
    let horizontal = Point::new(viewport_width, 0.0, 0.0);
    let vertical = Point::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Point::new(0.0, 0.0, focal_len);

    for j in (0..height).rev() { 
        for i in 0..width { 
            let u = i as f64 / ((width - 1) as f64);
            let v = j as f64 / ((height - 1) as f64);
            let ray = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let color_64 = ray_color(&ray);
            let color = convert_color(&color_64);
            canvas.buffer[j][i] = color;
        }
    }
}

fn convert_color(color_64 : &Vec3<f64>) -> Color { 
    Color { 
        x : (color_64.x * 255.999) as u8,
        y : (color_64.y *255.999) as u8,
        z : (color_64.z *255.999) as u8
    }
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