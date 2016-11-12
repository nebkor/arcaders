extern crate sdl2;
mod events;

use sdl2::pixels::Color;
use events::Events;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();


    // prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();
        println!("Pumped!");

        if events.quit || events.key_escape {
            break;
        }

        // Render a fully blue window
        renderer.set_draw_color(Color::RGB(0, 0, 255));
        renderer.clear();
        renderer.present();
    }

}