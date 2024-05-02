use piston_window::{color::*, types::Color, Context, G2d};

use crate::draw::{draw_block, CELL_SIZE};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Value {
    Empty,
    Block {color: Color}
}
pub struct Grid {
    rows: u32,
    cols: u32,
    cells: Vec<Vec<Value>>,
}

impl Grid {
    pub fn new(size: [u32;2]) -> Self {

        println!("{:?}", size);
        
        let rows = size[1]/(CELL_SIZE as u32);
        let cols = size[0]/(CELL_SIZE as u32);
        
        let mut cells = Vec::new();
        for _ in 0..rows {
            cells.push(new_row(cols));
        };

        println!("ROWS {:?}", cells.len());
        println!("COLS {:?}", cells[0].len());

        Self {
            rows,
            cols,
            cells,
        }
    }

    pub fn draw(&self, factor: [u32;2], con: &Context, g: &mut G2d) {

        for row in 0..self.rows {
            for col in 0..self.cols {

                let x = col + factor[0];
                let y = row + factor[1];

                match self.cells[row as usize][col as usize] {
                   Value::Empty => draw_block(GRAY, [x as f64, y as f64], con, g),
                   Value::Block{color} => draw_block(color, [x as f64, y as f64], con, g),
                }

                
            }
        }
    }

    pub fn update(&mut self, coords: Vec<[i32;2]>, color: Color) -> u32 {
        let mut score = 0;
        for i in 0..4 {
            let x = coords[i][0] as usize;
            let y = coords[i][1] as usize;

            self.cells[y][x] = Value::Block { color };

        }

        for row in (0..self.rows).rev() {


            // let x = coords[i][0] as usize;
            // let row = coords[i][1] as usize;

            // println!("{:?}", row);
            // println!("Check Row {:?}", self.check_row(row as usize));
        
            while self.check_row(row as usize) {
                self.update_rows(row as usize);
                score += 1;
            }
        }

        score
    }
    
    fn check_row(&self, row: usize) -> bool {
        for c in 0..self.cols {
            if self.cells[row][c as usize] == Value::Empty {
                return false;
            }
        }

        true
    }

    fn update_rows(&mut self, row: usize){

        // let new_row: Vec<Value>;
        for i in (1..row+1).rev() {

            // print!("i --- {:?}",i);

            // println!("PREV ROW {:?}", self.cells[i-1]);
            // println!("CURR ROW {:?}", self.cells[i]);

            self.cells[i] = self.cells[i-1].clone();


        }
    }

    pub fn check_valid(&self, coords: Vec<[i32;2]>) -> bool {
        if coords.len() != 4 {
            return false;
        }

        for i in 0..4 {
            if self.cells[coords[i][1] as usize][coords[i][0] as usize] != Value::Empty {
                return false;
            }
        }

        true
    }

}

fn new_row(cols: u32) -> Vec<Value> {

    let mut row = Vec::new();
    for _ in 0..cols {
        row.push(Value::Empty);
    };

    row
}
