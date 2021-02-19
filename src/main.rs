mod lib;

use lib::vec3::Color;
use lib::canvas::{Canvas, CanvasRender, PpmFile};

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
    let img_width : usize= 256; 
    let img_height : usize = 256; 
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

    let mut pixel_buff = { 
        let window_size = window.inner_size(); 
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(img_width as u32, img_height as u32, surface_texture).unwrap()
    };

    draw_gradient(&mut canvas);

    pixel_buff.render_canvas(&mut canvas).unwrap(); 
    pixel_buff.render().unwrap(); 


    event_loop.run(move |event, _, control_flow| {


        if input.update(&event) { 
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit(){
                *control_flow = ControlFlow::Exit; 
                return; 
            }
        }
    });

     

    // out_file.render(&canvas).unwrap(); 
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