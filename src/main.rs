extern crate sdl2;
extern crate time;
mod events;

use sdl2::pixels::Color;
use events::Events;

use std::time::Instant;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build()
        .unwrap();


    // prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    let mut loop_count = 1;
    let mut loop_cycle = 0;
    let loop_period = 1000;

    let now = Instant::now();

    loop {
        events.pump();
        if loop_count % loop_period == 0 {
            loop_cycle += 1;
            println!("Pumped {} cycle.", loop_cycle);
        }

        if events.quit || events.key_escape {
            let td = time::Duration::from_std(now.elapsed()).unwrap();
            let elapsed = td.num_milliseconds() as f64 / 1_000.0;
            let loops = loop_count;
            println!("{} loops at {} loops/second.",
                     loops,
                     loops as f64 / elapsed as f64);
            break;
        }

        // Render a fully blue window
        renderer.set_draw_color(Color::RGB(0, 0, 255));
        renderer.clear();
        renderer.present();

        loop_count += 1;
    }

}
