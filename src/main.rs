use draw::to_cell;
use game::Game;
use piston_window::{clear, Button, PistonWindow, PressEvent, ReleaseEvent, UpdateEvent, WindowSettings};
mod draw;
mod tetromino;
mod grid;
mod game;


const WINDOW_SIZE: [f64;2] = [20.0,30.0];

fn run_window() {
    let mut window: PistonWindow = WindowSettings::new("Tetris", [to_cell(WINDOW_SIZE[0]), to_cell(WINDOW_SIZE[1])])
        .exit_on_esc(true)
        .build()
        .unwrap();


    
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("NovaMono-Regular.ttf")).unwrap();
    
    let mut game = Game::new([to_cell(WINDOW_SIZE[0]) as u32, to_cell(WINDOW_SIZE[1]) as u32]);

    while let Some(event) = window.next() {

        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(Button::Keyboard(_)) = event.release_args() {
            game.key_released();
        }

        window.draw_2d(&event, |con, g, device| {

            clear([1.0;4], g);

            game.draw(&con, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }

}

fn main() {
    run_window();
}
