use piston_window::{self, color::{*}, types::Color, Context, G2d};

use crate::draw::draw_block;

const PINK: Color = [1.0,0.3,1.0,1.0];
const ORANGE: Color = [1.0,0.4,0.3,1.0];

#[derive(Debug)]
pub struct Tetromino {
    color: Color,
    position: [f64;2],
    coords: Vec<[i32;2]>
}

impl Tetromino {
    pub fn new_o() -> Self {
        Self  { 
            color: YELLOW,
            position: [1.0,1.0],
            coords: vec![
                [0,0],
                [1,0],
                [0,1],
                [1,1]
            ]
         }
    }

    pub fn new_i() -> Self {
        Self {
            color: CYAN,
            position: [1.0,1.0],
            coords: vec![
                [0,0],
                [0,1],
                [0,2],
                [0,3]
            ]
        }
    }

    pub fn new_j() -> Self {
        Self {
            color: PINK,
            position: [1.0,1.0],
            coords: vec![
                [0,2],
                [1,0],
                [1,1],
                [1,2]
            ]
            
        }
    }

    pub fn new_s() -> Self {
        Self {
            color: RED,
            position:[1.0,1.0],
            coords: vec![
                [0,1],
                [1,1],
                [1,0],
                [2,0],
            ]
        }
    }

    pub fn new_z() -> Self {
        Self {
            color: GREEN,
            position:[1.0,1.0],
            coords: vec![
                [0,0],
                [1,0],
                [1,1],
                [2,1],
            ]
        }
    }

    pub fn new_t() -> Self {
        Self {
            color: PURPLE,
            position:[1.0,1.0],
            coords: vec![
                [0,0],
                [1,0],
                [2,0],
                [1,1],
            ]
        }
    }

    pub fn new_l() -> Self {
        Self {
            color: ORANGE,
            position:[1.0,1.0],
            coords: vec![
                [0,0],
                [0,1],
                [0,2],
                [1,2],
            ]
        }
    }

    pub fn draw(&mut self, factor:[i32;2], con: &Context, g: &mut G2d) {
        for i in 0..4 {
            let x = self.coords[i][0] + factor[0];
            let y = self.coords[i][1] + factor[1];

            draw_block(self.color, [x as f64, y as f64], con, g)
        }
    }
    
    pub fn get_color(&self) -> Color {
        return self.color
    }

    pub fn get_coords(&self) -> Vec<[i32;2]> {
        self.coords.to_owned()
    }

    pub fn set_coords(&mut self, center: [i32;2]) {
        for i in 0..4 {
            self.coords[i][0] += center[0];
            self.coords[i][1] += center[1];
        }
    }

    pub fn get_next_drop_coords(&mut self) -> Vec<[i32; 2]>{
        let mut next_coords: Vec<[i32;2]> = Vec::new();

        for i in 0..4 {
            next_coords.push([self.coords[i][0], self.coords[i][1] + 1]);
        }

        next_coords
    }


    pub fn get_next_left_coords(&mut self, min_x: i32) -> Vec<[i32; 2]>{
        let mut next_coords: Vec<[i32;2]> = Vec::new();

        for i in 0..4 {

            if self.coords[i][0] <= min_x {
                return Vec::new();
            }
            next_coords.push([self.coords[i][0] - 1, self.coords[i][1]]);
        }

        next_coords
    }

    pub fn get_next_right_coords(&mut self, max_x: i32) -> Vec<[i32; 2]>{
        let mut next_coords: Vec<[i32;2]> = Vec::new();

        for i in 0..4 {

            if self.coords[i][0] >= max_x {
                return Vec::new()
            }
            next_coords.push([self.coords[i][0] + 1, self.coords[i][1]]);
        }

        next_coords
    }

    pub fn update_position(&mut self, position: [i32;2]) {
        let x = position[0];
        let y = position[1];

        for i in 0..4 {
            self.coords[i][0] = self.coords[i][0] as i32 + x;
            self.coords[i][1] = self.coords[i][1] as i32 + y;
        }
    }

    pub fn drop(&mut self) {
        self.update_position([0,1]);
    }

    pub fn move_left(&mut self, min_x: i32) {
        // println!("COORDS {:?}",self.coords);
        for i in 0..4 {
            if self.coords[i][0] <= min_x {
                return;
            }
        }
        self.update_position([-1,0]);
    }

    pub fn move_right(&mut self, max_x: i32) {

        for i in 0..4 {
            if self.coords[i][0] >= max_x {
                return;
            }
        }

        self.update_position([1,0]);
    }


}




