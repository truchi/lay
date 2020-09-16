use super::Styled;
use crate::Cell;

pub trait Layer {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn get(&self, x: u16, y: u16) -> Option<Cell>;
}

pub trait LayerMut: Layer {
    fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell>;

    fn above<T: Layer>(&mut self, above: &T, x: u16, y: u16) {
        merge(self, above, x, y, Cell::above)
    }

    fn below<T: Layer>(&mut self, below: &T, x: u16, y: u16) {
        merge(self, below, x, y, Cell::below)
    }
}

fn merge<T: LayerMut + ?Sized, U: Layer>(
    a: &mut T,
    b: &U,
    x: u16,
    y: u16,
    f: fn(&Cell, &Cell) -> Cell,
) {
    let x2 = a.width().min(x + b.width());
    let y2 = a.height().min(y + b.height());

    for row in x..x2 {
        for line in y..y2 {
            if let Some(cell_a) = a.get_mut(row, line) {
                if let Some(cell_b) = b.get(row - x, line - y) {
                    *cell_a = f(cell_a, &cell_b);
                }
            }
        }
    }
}

impl Layer for Cell {
    fn width(&self) -> u16 {
        1
    }

    fn height(&self) -> u16 {
        1
    }

    fn get(&self, x: u16, y: u16) -> Option<Cell> {
        if x == 0 && y == 0 {
            Some(*self)
        } else {
            None
        }
    }
}

impl LayerMut for Cell {
    fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x == 0 && y == 0 {
            Some(self)
        } else {
            None
        }
    }
}

macro_rules! layer_str {
    ($($StrType:ty,)*) => {
        $(
            impl Layer for Styled<$StrType> {
                fn width(&self) -> u16 {
                    self.content.len() as u16
                }

                fn height(&self) -> u16 {
                    1
                }

                fn get(&self, x: u16, y: u16) -> Option<Cell> {
                    if y == 0 {
                        self.content
                            .chars()
                            .nth(x as usize)
                            .map(|c| (c, self.style).into())
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

layer_str!(String, &str,);
