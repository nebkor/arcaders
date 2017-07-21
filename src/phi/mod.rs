#[macro_use]
mod events;
pub mod data;
pub mod gfx;

use views::View;

use sdl2::render::Renderer;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_space: Space,
        key_left: Left,
        key_right: Right
    },
    else: {
        quit: Quit { .. }
    }
}


/// Bundles the Phi abstractions in a single structure which
/// can be passed easily between functions.
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        Phi {
            events: events,
            renderer: renderer,
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }
}


/// A `ViewAction` is a way for the currently executed view to
/// communicate with the game loop. It specifies which action
/// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

/// Create a window with name `title`, initialize the underlying libraries and
/// start the game with the `View` returned by `init()`.
///
pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View>
{
    // Initialize SDL2
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _image_context = ::sdl2::image::init(::sdl2::image::INIT_PNG).unwrap();

    // Create the window
    let window = video.window(title, 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create the context
    let mut context = Phi::new(Events::new(sdl_context.event_pump().unwrap()),
                               window.renderer()
                                   .accelerated()
                                   .build()
                                   .unwrap());

    // Create the default view
    let mut current_view = init(&mut context);


    // Frame timing

    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = before;
    let mut fps = 0u16;

    loop {
        // Frame timing (bits)
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        // If the time elapsed since the last frame is too small, wait out the
        // difference and try again.
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 999 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }


        // Logic & rendering

        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => current_view = new_view,
        }
    }
}
