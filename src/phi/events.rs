// extern crate sdl2;

macro_rules! struct_events {
    (
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },

        // match against a pattern
        else: { $( $e_alias:ident : $e_sdl:pat ),* }
    ) => {
        use sdl2::EventPump;

        pub struct ImmediateEvents {
            // for every keyboard event, we have an Option<bool>
            // Some(true) => was just pressed
            // Some(false) => was just released
            // None => nothing happening right now.
            $( pub $k_alias: Option<bool> , )*
                $( pub $e_alias : bool ),* ,
            resize: Option<(u32, u32)>
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    // when initialized, nothing has happened yet, so all are
                    // set to None
                    $( $k_alias: None , )*
                    $( $e_alias: false ),* ,
                    resize: None
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            // true => pressed
            // false => not pressed
            $( pub $k_alias: bool ),*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    // by default, initialize evy key with not pressed
                    $( $k_alias: false ),*
                }
            }

            pub fn pump(&mut self, renderer: &mut ::sdl2::render::Renderer) {
                self.now = ImmediateEvents::new();
                use sdl2::event::Event::*;
                use sdl2::event::WindowEvent::*;
                use sdl2::keyboard::Keycode::*;

                for event in self.pump.poll_iter() {
                    match event {
                        Window { window_id, win_event: Resized(x, y), .. } => {
                            println!("Resized window {} to {}, {}", window_id, x, y);
                            self.now.resize = Some(renderer.output_size().unwrap());
                    },
                        
                        KeyDown { keycode, .. } => match keycode {
                            // $( .. ),* containing $k_sdl and $k_alias means:
                            // "for every element ($k_alias : $k_sdl) pair,
                            // check whether the keycode is Some($k_sdl). If
                            // it is, then set the $k_alias fields to true."
                            $(
                                Some($k_sdl) => {
                                    // prevent multiple presses when keeping a
                                    // key down; was it previously pressed or not?
                                    if !self.$k_alias {
                                        // key pressed
                                        self.now.$k_alias = Some(true);
                                    }
                                    self.$k_alias = true;
                                }
                            ),*
                                _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // key released
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                            ),*
                                _ => {}
                        },

                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            }
                        )*,

                        _ => {}
                    }
                }
            }
        }
    }
}
