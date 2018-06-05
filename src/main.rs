extern crate piston_window;
extern crate rand;

mod tetris;
mod piece;
mod data;

use piston_window::*;
use tetris::Tetris;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tetris", [400, 600])
        .build()
        .unwrap();

    window.window.set_position([(1366 - 400) / 2, (768 - 640) / 2]);

    let mut elapsed = 0.0;
    let mut tetris = Tetris::new();

    while let Some(event) = window.next() {
        match event {
            Event::Loop(Loop::Update(args)) => {
                elapsed += args.dt;
                if elapsed > 0.5 {
                    elapsed = 0.0;
                    tetris = tetris.tick();
                }
            },
            Event::Input(Input::Button(args)) => {
                if let (ButtonState::Release, Button::Keyboard(key)) = (args.state, args.button) {
                    tetris = tetris.key(key);
                }
            },
            _ => ()
        }

        window.draw_2d(&event, |context, graphics| {
            tetris.draw(&context, graphics);
        });
    }
}
