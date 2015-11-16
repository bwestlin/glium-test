extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate rand;

use glium_sdl2::DisplayBuild;
use rand::Rng;
use rand::thread_rng;
use glium::Surface;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display = video_subsystem.window("My window", 800, 600)
        .resizable()
        .build_glium()
        .unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut rng = thread_rng();

    while running {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let texture = glium::texture::Texture2d::empty(&display, 2, 2).unwrap();
        let buffer = glium::texture::pixel_buffer::PixelBuffer::new_empty(&display, 4);
        buffer.write(&[(rng.next_u32() as u8, rng.next_u32() as u8, rng.next_u32() as u8, 255u8), (255, 0, 255, 0), (255, 255, 0, 255),
                       (0, 0, 255, 255)]);
        texture.main_level().raw_upload_from_pixel_buffer(buffer.as_slice(), 0 .. 2, 0 .. 2, 0 .. 1);

        texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);

        target.finish().unwrap();

        // Event loop: includes all windows
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit { .. } => {
                    running = false;
                },
                _ => ()
            }
        }
    }
}
