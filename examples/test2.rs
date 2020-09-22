use crossterm::{
    cursor::{position, MoveTo},
    terminal::size,
};
use lay::*;
use std::{thread::sleep, time::Duration};

fn main() {
    let (width, _) = size().unwrap();
    let width = width / 2;
    let (_, y) = position().unwrap();

    let background = Cell::from('💧') / Blue;
    let crab = Cell::from('🦀') * Reset;
    let mut canvas = Canvas::with_cell(background, width, 1);

    loop {
        let mut prev = width - 1;
        for i in 0..width {
            canvas >>= (&background, prev, 0);
            canvas >>= (&crab, i, 0);
            prev = i;
            println!("{}{}", Render::new(&canvas, 0, y), MoveTo(0, y - 1));
            sleep(Duration::from_millis(100));
        }
    }
}
