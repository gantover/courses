use crate::{NUM_COLS, NUM_ROWS};

// frame module
pub type Frame = Vec<Vec<&'static str>>; // type alias

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS); //outer vector
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_COLS {
            col.push(" "); // blank frame
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
