use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::Sprite;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureQuery};
use sdl2::image::LoadTexture;

use std::path::Path;

// Constants
const PLAYER_SPEED: f64 = 180.0;

const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

// Data types
struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
}

/// The different states our ship might be in. In the image, they're ordered
/// from left to right, then from top to bottom.
#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm = 0,
    UpFast = 1,
    UpSlow = 2,
    MidNorm = 3,
    MidFast = 4,
    MidSlow = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8,
}


// View definitions

pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        // ? Load the texture from the filesystem.
        // ? If it cannot be found, then there is no point in continuing: panic!
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();

        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                        w: SHIP_W,
                        h: SHIP_H,
                        x: SHIP_W * x as f64,
                        y: SHIP_H * y as f64,
                    })
                    .unwrap());
            }
        }

        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H,
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
            },
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // View logic here

        phi.renderer.set_draw_color(Color::RGB(0, 0, 255));
        phi.renderer.clear();

        // View rendering here

        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        match phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap()) {
            Ok(_) => {}
            Err(e) => {
                println!("Got error {} from fill_rect", e);
            }
        };

        // render the sprite


        let diagonal = (phi.events.key_up ^ phi.events.key_down) &&
                       (phi.events.key_left ^ phi.events.key_right);

        let moved = if diagonal { 1.0 / 2.0f64.sqrt() } else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        // The movable region spans the entire height of the window and 70% of its
        // width. This way, the player cannot get to the far right of the screen, where
        // we will spawn the asteroids, and get immediately eliminated.
        //
        // We restrain the width because most screens are wider than they are high.
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };

        // If the player cannot fit in the screen, then there is a problem and
        // the game should be promptly aborted.
        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        self.player.current = if dx == 0.0 && dy < 0.0 {
            ShipFrame::UpNorm
        } else if dx > 0.0 && dy < 0.0 {
            ShipFrame::UpFast
        } else if dx < 0.0 && dy < 0.0 {
            ShipFrame::UpSlow
        } else if dx == 0.0 && dy == 0.0 {
            ShipFrame::MidNorm
        } else if dx > 0.0 && dy == 0.0 {
            ShipFrame::MidFast
        } else if dx < 0.0 && dy == 0.0 {
            ShipFrame::MidSlow
        } else if dx == 0.0 && dy > 0.0 {
            ShipFrame::DownNorm
        } else if dx > 0.0 && dy > 0.0 {
            ShipFrame::DownFast
        } else if dx < 0.0 && dy > 0.0 {
            ShipFrame::DownSlow
        } else {
            unreachable!()
        };

        self.player.sprites[self.player.current as usize]
            .render(&mut phi.renderer, self.player.rect);

        ViewAction::None
    }
}
