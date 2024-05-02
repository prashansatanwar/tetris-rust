use piston_window::{color::*, types::Color, Context, G2d, Key, Glyphs};
use rand::Rng;
use crate::draw::{draw_text, draw_rectangle, to_cell, CELL_SIZE};
use crate::tetromino::Tetromino;
use crate::grid::Grid;

pub const MARGIN_TOP: f64 = 2.0;
const GAME_OVER_COLOR: Color = [0.0, 0.0, 0.0, 0.9];

enum Direction {
    Left,
    Right,
    Down
}

pub struct Game {
    game_over: bool,

    falling_tetromino: Tetromino,
    drop_timer: f64,
    drop_interval: f64,
    dropped: bool,

    move_timer: f64,
    move_interval: f64,
    
    grid: Grid,
    size: [f64;2],

    active_key: Option<Key>,

    score: u32,

}

impl Game {
    pub fn new(size: [u32;2]) -> Self {
        Self{
            game_over: false,
            falling_tetromino: get_falling_tetromino([size[0] as f64, size[1] as f64]),
            drop_timer: 0.0,
            drop_interval: 1.0,
            dropped: false,

            move_timer: 0.0,
            move_interval:0.1,

            grid: Grid::new(size),
            size: [size[0] as f64, size[1] as f64],

            active_key: None,

            score: 0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Left => self.active_key = Some(key),
            Key::Right => self.active_key = Some(key),
            Key::Down => self.active_key = Some(key),
            _ => self.active_key = None
        }
    }

    pub fn key_released(&mut self) {
        self.active_key = None;
    }

    pub fn draw(&mut self, con: &Context, g: &mut G2d, cache: &mut Glyphs) {

        draw_rectangle(BLACK, [0.0,0.0], [self.size[0], to_cell(MARGIN_TOP)], con, g);
        self.grid.draw([0, MARGIN_TOP as u32] ,con, g);
        self.falling_tetromino.draw([0, MARGIN_TOP as i32], con, g);

        if self.game_over {
            println!("{:?}",self.size[1]/(4.0*CELL_SIZE));
            draw_rectangle(GAME_OVER_COLOR, [1.5,(self.size[1]/(4.0*CELL_SIZE)+MARGIN_TOP)], [self.size[0] - (CELL_SIZE * 4.0), (self.size[1]/2.0)-(CELL_SIZE * 4.0)], con, g);
            draw_text(format!("GAME OVER").as_str(), [7.0,(self.size[1]/(4.0*CELL_SIZE)+(4.0* MARGIN_TOP))],30, con, g, cache);
        }

        draw_text(format!("Score: ").as_str(), [0.5,1.0],20, con, g, cache);
        draw_text(format!("{}", self.score).as_str(), [3.5,1.0],20, con, g, cache);

    }

    pub fn update(&mut self, dt: f64) {

        self.drop_timer += dt;
        self.move_timer += dt;

        
        self.update_falling_tetromino(dt, self.get_dir());
    }

    fn update_falling_tetromino(&mut self, dt: f64, direction:Option<Direction>) {
        let max_y = ((self.size[1]/CELL_SIZE) - MARGIN_TOP) as i32;
        let coords = self.falling_tetromino.get_coords();
        let mut next_position = self.falling_tetromino.get_next_drop_coords();

        let min_x = 0;
        let max_x = (self.size[0]/CELL_SIZE) as i32 - 1;

        let is_valid = self.grid.check_valid(next_position.clone());

        for i in 0..4 {
            if !is_valid || next_position[i][1] >= max_y {
                self.dropped = true;
                self.drop_timer += self.drop_interval;
                self.move_timer = 0.0;
            }
        }

        if self.move_timer >= self.move_interval {
            if let Some(dir) = direction {
                match dir {
                    Direction::Left => {
                        next_position = self.falling_tetromino.get_next_left_coords(min_x);
                        let is_valid = self.grid.check_valid(next_position.clone());

                        if is_valid {
                            self.falling_tetromino.move_left(min_x)
                        }
                    },
                    Direction::Right => {
                        next_position = self.falling_tetromino.get_next_right_coords(max_x);
                        let is_valid = self.grid.check_valid(next_position.clone());

                        if is_valid {
                            self.falling_tetromino.move_right(max_x)
                        }
                    },
                    Direction::Down => {
                        let is_valid = self.grid.check_valid(next_position.clone());
                        if is_valid {
                            self.falling_tetromino.drop();
                        }
                    }
                }
                self.move_timer = 0.0;
                next_position = self.falling_tetromino.get_next_drop_coords();
            }
        }

        
        if self.drop_timer >= self.drop_interval  {
            if self.dropped {
                self.score += self.grid.update(coords.clone(), self.falling_tetromino.get_color());
                let temp = get_falling_tetromino(self.size);
                let temp_coords = temp.get_coords();
                if self.grid.check_valid(temp_coords) {
                    self.falling_tetromino = temp;
                    self.dropped = false;
                }
                else {
                    self.game_over = true;
                }
            }
            else {
                self.falling_tetromino.drop();
            }
            self.drop_timer = 0.0;
        }
    }

    fn get_dir(&self) -> Option<Direction> {
        let mut direction: Option<Direction> = None;

        if self.active_key.is_some() {   
            match self.active_key {
                Some(Key::Left) => direction = Some(Direction::Left),
                Some(Key::Right) => direction = Some(Direction::Right),
                Some(Key::Down) => direction = Some(Direction::Down),
                _ => direction = None
            }
        }
        
        direction
    }
}

fn get_falling_tetromino(size:[f64;2]) -> Tetromino {
    let shapes = ["O", "I", "S", "Z", "L", "J", "T"];

    let mut rng = rand::thread_rng();
    let random_ind = rng.gen_range(0..7);
    let random_shape = shapes[random_ind];

    let mut tetromino_new: Tetromino;

    match random_shape {
        "O" => tetromino_new = Tetromino::new_o(), 
        "I" => tetromino_new = Tetromino::new_i(),
        "S" => tetromino_new = Tetromino::new_s(),
        "Z" => tetromino_new = Tetromino::new_z(),
        "L" => tetromino_new = Tetromino::new_l(),
        "J" => tetromino_new = Tetromino::new_j(),
        "T" => tetromino_new = Tetromino::new_t(),
        _ => tetromino_new = Tetromino::new_o()
    }

    tetromino_new.set_coords([(size[0]/(2.0*CELL_SIZE)) as i32 -2 ,0]);

    tetromino_new
}
