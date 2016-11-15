// extern crate sdl2;

macro_rules! struct_events {
    ( /* PATTERN */ ) => {
        use sdl2::EventPump;

        pub struct ImmediateEvents;

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new()
                }
            }

            pub fn pump(&mut self) {

                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        _ => {}
                    }
                }
            }
        }
    }
}
