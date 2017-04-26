use phi::{Phi, View, ViewAction, Events};
use sdl2::pixels::Color;

pub struct ViewA;
pub struct ViewB;

fn do_quit(e: &Events) -> bool {
    if e.now.quit || e.now.key_escape == Some(true) || e.now.key_q == Some(true) {
        return true;
    }
    false
}

impl View for ViewA {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if do_quit(events) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewB));
        }

        renderer.set_draw_color(Color::RGB(0, 0, 255));
        renderer.clear();

        ViewAction::None
    }
}

impl View for ViewB {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if do_quit(events) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewA));
        }

        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.clear();

        ViewAction::None
    }
}
