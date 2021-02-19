mod lib;

use lib::vec3::Color as LibColor;
use lib::canvas::{Canvas as LibCanvas, CanvasRender, PpmFile};

use sdl2::{
    pixels::Color, 
    video::Window, 
    render::Canvas,
    rect::Point,
    event::Event,
    keyboard::Keycode
};


struct Sdl2Renderer { 
    inner : Canvas<Window>
}

impl Sdl2Renderer { 
    pub fn new(ctx : &sdl2::Sdl, width : usize, height : usize) -> Self { 

        let video_subsystem = ctx.video().unwrap();
    
        let window = video_subsystem.window("rust-sdl2 demo",width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();


        Sdl2Renderer { 
            inner : canvas 
        }
    }
}

impl CanvasRender for Sdl2Renderer { 
    
fn render(&mut self, canvas: &LibCanvas<u8>) ->Result<(), std::io::Error> { 

    canvas.buffer.iter().enumerate().for_each(|(y_idx, row)| {
        row.iter().enumerate().for_each(|(x_idx, color)| {
            self.inner.set_draw_color(Color::from((color.x, color.y, color.z)));

            self.inner.draw_point(Point::from((y_idx as i32, x_idx as i32))).unwrap();
        });
    });

    self.inner.present();

    Ok(())
}
} 

fn main() {
    let img_width : usize= 256; 
    let img_height : usize = 256; 
    let mut out_file = PpmFile::create("public/out.ppm").unwrap(); 
    let mut canvas = LibCanvas::<u8>::new(img_width, img_height);

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut renderer = Sdl2Renderer::new(&sdl_context, img_width, img_height);

    draw_gradient(&mut canvas);

    renderer.render(&canvas).unwrap(); 

    'running:  loop { 

        for event in event_pump.poll_iter() { 
            match event { 
                Event::Quit{..} |
                Event::KeyDown{keycode : Some(Keycode::Escape), ..} => { break 'running}
            _ => {}
            }
        }

    }

    out_file.render(&canvas).unwrap(); 
}



fn draw_gradient(canvas : &mut LibCanvas<u8>) { 
    let img_height = canvas.height; 
    let img_width = canvas.width; 

    for i in (0..img_height).rev() { 
        for j in 0..img_width { 
            let red = i as f64/ (img_width - 1) as f64; 
            let blue = j as f64 / (img_width - 1) as f64; 
            let green = 0.25 * 255.0;

            let color = LibColor { 
                x : (red * 255.999) as u8,
                y : (blue *255.999) as u8,
                z : green as u8
            }; 
            canvas.buffer[i][j] = color;
        };
    };
}