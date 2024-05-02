use piston_window::{rectangle, text, types::Color, color::* , Context, G2d, Glyphs, Transformed};

// pub const BLOCK_SIZE: f64 = 10.0;
pub const CELL_SIZE: f64 = 30.0;
const FONT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

// pub fn to_coordinate(coordinate: f64) -> f64 {
//     coordinate * BLOCK_SIZE
// }

pub fn to_cell(n: f64) -> f64 {
    n * CELL_SIZE
}

pub fn draw_block(color: Color, position: [f64;2], con: &Context, g: &mut G2d) {
    let x = to_cell(position[0]);
    let y = to_cell(position[1]);

    rectangle(
        BLACK,
        [x - 1.5, y - 1.5, CELL_SIZE + 3.0, CELL_SIZE + 3.0], // Border rectangle
        con.transform,
        g,
    );

    rectangle(
        color,
        [x,y,CELL_SIZE,CELL_SIZE],
        con.transform,
        g,
    )
}

pub fn draw_rectangle(color: Color, position: [f64;2], size: [f64;2], con: &Context, g: &mut G2d) {
    let x = to_cell(position[0]);
    let y = to_cell(position[1]);

    rectangle(
        color,
        [x,y,CELL_SIZE + size[0],CELL_SIZE + size[1]],
        con.transform,
        g,
    )

}

pub fn draw_text(text: &str, position: [f64; 2], font_size: u32, con: &Context, g: &mut G2d, cache: &mut Glyphs) {
    let transform = con.transform.trans(to_cell(position[0]), to_cell(position[1]));

    text::Text::new_color(FONT_COLOR, font_size)
        .draw(text, cache, &con.draw_state, transform, g)
        .unwrap();
}