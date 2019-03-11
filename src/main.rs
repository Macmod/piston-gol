extern crate piston_window;

use piston_window::*;

// Basic example which does not work yet.
fn main() {
    const SQUARE: f64 = 30.0;

    let mut window: PistonWindow =
        WindowSettings::new("Game of Life", [640, 480])
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            let mut x = SQUARE;
            let mut y = SQUARE;
            while x <= 610.0 {
                while y <= 450.0 {
                    rectangle([1.0, 0.0, 0.0, 1.0],
                              [x - SQUARE, y - SQUARE, x, y],
                              context.transform,
                              graphics);

                    y += SQUARE;
                }

                x += SQUARE;
            }
        });
    }
}
