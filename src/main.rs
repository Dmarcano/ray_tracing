mod lib;

use lib::vec3::Color;
use lib::canvas::{Canvas, CanvasRender, PpmFile};

use winit::{ 
    event::*, 
    event_loop::{EventLoop, ControlFlow}, 
    window::{Window, WindowBuilder},
};

fn main() {
    env_logger::init(); 
    let event_loop = EventLoop::new(); 
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event { 
            Event::WindowEvent {
                ref event, 
                window_id
            } if window_id == window.id() => match event { 
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit, 
                WindowEvent::KeyboardInput { 
                    input, 
                    .. 
                    } => {
                        match input {
                            KeyboardInput {
                                state : ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    }
                _ => {}
                }
                _ => {} 
        }
    })
    
}




fn ppm_example() { 
    let img_width : usize= 256; 
    let img_height : usize = 256; 
    let mut out_file = PpmFile::create("public/out.ppm").unwrap(); 
    let mut canvas = Canvas::<u8>::new(img_width, img_height);

    draw_gradient(&mut canvas);
    out_file.render(&canvas).unwrap(); 
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