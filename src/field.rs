use crate::fill::*;

const WIN_COUNT : usize = 4;

pub struct Field {
    pub height: usize,
    pub width: usize,

    state: Box<[Fill]>,
}

impl Field {

    pub fn new(height: usize, width: usize) -> Field {
        return Field{
            height: height,
            width: width,
            state: vec![Fill::Empty; height * width].into_boxed_slice()
        }
    }  

    pub fn get_fill(&self, x: usize, y: usize) -> Fill {

        if self.is_out_of_bounds(x, y) {
            return Fill::OutOfBounds;
        }

        return self.state[self.translate_to_index(x, y)];
    }

    pub fn insert(&mut self, x: usize, fill: Fill) -> i32 {
        for y in 0..(self.height) {
            //let y = self.height - i - 1;
            
            if self.get_fill(x, y) == Fill::Empty {
                self.set_fill(x, y, fill);
                return y as i32;
            }
        }

        return -1;
    }

    fn set_fill(&mut self, x: usize, y: usize, fill: Fill) {
        if self.is_out_of_bounds(x, y) {
            return;
        }

        self.state[self.translate_to_index(x, y)] = fill;
    }

    fn is_out_of_bounds(&self, x: usize, y: usize) -> bool {
        return x >= self.width || y >= self.height;
    }

    fn translate_to_index(&self, x: usize, y: usize) -> usize {
        return x as usize * self.width + y as usize;
    }

    pub fn get_winner(&self) -> i32 {
        for y in 0..=self.height - WIN_COUNT {
            for x in 0..=self.width - WIN_COUNT {
                let fill = self.find_winner_at_block(x, y);
                if fill != Fill::Empty {
                    return fill_enum_to_player_index(fill);
                }
            } 
        }

        return -1;
    } 

    fn find_winner_at_block(&self, x: usize, y: usize) -> Fill {
        let fill: Fill = self.get_fill(x, y);

        if fill != Fill::Player1 && fill != Fill::Player2 {
            return Fill::Empty;
        }

        if self.find_winner_horizontally(fill, x, y) {
            return fill;
        }

        if self.find_winner_vertically(fill, x, y) {
            return fill;
        }

        if self.find_winner_diagonally(fill, x, y) {
            return fill;
        }
        
        return Fill::Empty;
    }

    fn find_winner_horizontally(&self, fill: Fill, x: usize, y: usize) -> bool {

        for i in 1..WIN_COUNT {
            
            if self.get_fill(x + i, y) != fill {
                return false;
            }
        }

        return true;
    }

    fn find_winner_vertically(&self, fill: Fill, x: usize, y: usize) -> bool {

        for i in 1..WIN_COUNT {
            
            if self.get_fill(x, y + i) != fill {
                return false;
            }
        }

        return true;
    }

    fn find_winner_diagonally(&self, fill: Fill, x: usize, y: usize) -> bool {

        for i in 1..WIN_COUNT {
            
            if self.get_fill(x + i, y + i) != fill {
                return false;
            }
        }

        return true;
    }
}

fn fill_enum_to_player_index(fill: Fill) -> i32 {
    match fill {
        Fill::Player1 => 0,
        Fill::Player2 => 1,
        _ => -1,
    }
}