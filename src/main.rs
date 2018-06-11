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

    window.window.set_position([(1366 - 400) / 2, (768 - 600) / 2 - 20]);

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
                use ButtonState::Release;
                use Button::Keyboard;
                if let ButtonArgs { state: Release, button: Keyboard(key), .. } = args {
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
