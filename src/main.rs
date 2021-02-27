mod lib;

use lib::vec3::{Color, Vec3, Point};
use lib::canvas::{Canvas, CanvasRender, PpmFile};
use lib::ray::Ray; 

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{ 
    dpi::LogicalSize, 
    event::{Event, VirtualKeyCode}, 
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window}, 
};
use winit_input_helper::WinitInputHelper; 



impl CanvasRender for Pixels<Window> { 

    fn render_canvas(&mut self, canvas: &Canvas<u8>) -> Result<(), std::io::Error> { 
        let frame = self.get_frame(); 

        canvas.buffer.iter().flatten().enumerate().for_each(|(i, row)|{

            // the pixels buffer is a 4X len buffer of the canvas with each 4 indices corresponding to RGBA of a single pixel
            let buf_idx = 4*i; 
            frame[buf_idx] = row.x; // R
            frame[buf_idx + 1] = row.y; // G
            frame[buf_idx + 2] = row.z;  // B
            frame[buf_idx + 3] = 0xFF; // A 

        });
        Ok(())
    }
}

fn main() {
    // here we setup the virtual viewport for the aspect ratio of the image
    let aspect_ration = 16.0/9.0;
    // then we create an image that is at least x width and has a height in line with 
    // the aspect ratio
    let img_width : usize= 400; 
    let img_height : usize = (img_width as f64 / aspect_ration) as usize; 
    // This ppm file is one way the canvas can be rendered.
    let mut out_file = PpmFile::create("public/out.ppm").unwrap(); 
    let mut canvas = Canvas::<u8>::new(img_width, img_height);

    let event_loop = EventLoop::new(); 
    let mut input = WinitInputHelper::new(); 

    let window = {
        let size = LogicalSize::new(img_width as f64, img_height as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    // Here we create a 4*L*W buffer which is how we can render in real time.
    let mut pixel_buff = { 
        let window_size = window.inner_size(); 
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(img_width as u32, img_height as u32, surface_texture).unwrap()
    };

    // Here is where the ray tracer "ray traces" our canvas
    draw_lerp(&mut canvas);

    // and we render it out afterwards
    pixel_buff.render_canvas(&mut canvas).unwrap(); 
    pixel_buff.render().unwrap(); 
    // right now i comment and uncomment this line as I want to render files
    out_file.render_canvas(&canvas).unwrap();


    event_loop.run(move |event, _, control_flow| {


        if input.update(&event) { 
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit(){
                *control_flow = ControlFlow::Exit; 
                return; 
            }
        }
    });

    
}

// function for whether some ray has hit a sphere in our view.
fn hit_sphere(center : &Point, radius : f64 ,ray: &Ray) -> bool { 
    let oc = ray.origin - *center; 
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * Vec3::dot(&oc, &ray.direction);
    let c  = Vec3::dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - a*c*4.0;
    discriminant > 0.0
}

fn ray_color(ray : &Ray) ->Vec3<f64>{ 
    let unit_vec = ray.direction.unit_vec();
    let t = (unit_vec.y + 1.0)*0.5;

    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0),0.5, &ray) { 
        return Vec3::new(1.0, 0.0, 0.0)
    }
    // a linear gradient on the value t which is proportional to the y value of the vectors
    // start near blue when t is small and end at white when t is large. 
    Vec3::new(1.0, 1.0, 1.0)*t + Vec3::new(0.5, 0.7, 1.0)*(1.0-t)
}

// A quick linear interpolation that renders an image from white to blue based 
// on where in the y axis of the virtual viewport it is.
fn draw_lerp(canvas: &mut Canvas<u8>) { 
    let width = canvas.width; 
    let height = canvas.height; 

    // setup camera 
    let viewport_height = 2.0; 
    let viewport_width = 16.0/9.0 * viewport_height; 
    let focal_len = 1.0; 
    
    // setup the scene
    let origin = Point::zero();
    let horizontal = Point::new(viewport_width, 0.0, 0.0);
    let vertical = Point::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Point::new(0.0, 0.0, focal_len);

    // draw the gradient
    for j in (0..height).rev() { 
        for i in 0..width { 
            // u and v are vectors which are component vectors that make up the x and y 
            // components of our ray from the camera to the pixel that is currentli traced.
            let u = i as f64 / ((width - 1) as f64);
            let v = j as f64 / ((height - 1) as f64);
            let ray = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            // here in drawing our ray tracing we give it a function that takes in our 
            let color_64 = ray_color(&ray);
            let color = convert_color(&color_64);
            canvas.buffer[j][i] = color;
        }
    }
}

// abstract away converting from a floating point Vec3 to a uint8 
fn convert_color(color_64 : &Vec3<f64>) -> Color { 
    Color { 
        x : (color_64.x * 255.999) as u8,
        y : (color_64.y *255.999) as u8,
        z : (color_64.z *255.999) as u8
    }
}

// this is a sample function that draws a pure gradient of colors to test out 
// the rendering capabilities of any rederer. No ray tracing code here.
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