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

    window.window.set_position([1600 / 2 - 400 / 2, 900 / 2 - 600 / 2]);

    let mut elapsed = 0.0;
    let mut tetris = Tetris::new();

    while let Some(event) = window.next() {
        if let Some(update_args) = event.update_args() {
            elapsed += update_args.dt;
            if elapsed > 0.5 {
                elapsed = 0.0;
                tetris = tetris.tick();
            }
        }

        if let Some(button_args) = event.button_args() {
            if let ButtonState::Release = button_args.state {
                if let Button::Keyboard(key) = button_args.button {
                    tetris = tetris.key(key);
                }
            }
        }

        window.draw_2d(&event, |context, graphics| {
            tetris.draw(&context, graphics);
        });
    }
}
