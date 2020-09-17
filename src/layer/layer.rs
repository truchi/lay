use super::Cell;
use crate::Styled;

pub trait Layer {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn get_unchecked(&self, x: u16, y: u16) -> Cell;

    fn get(&self, x: u16, y: u16) -> Result<Cell, ()> {
        if x < self.width() && y < self.height() {
            Ok(self.get_unchecked(x, y))
        } else {
            Err(())
        }
    }
}

pub trait LayerMut: Layer {
    fn get_mut_unchecked(&mut self, x: u16, y: u16) -> &mut Cell;

    fn get_mut(&mut self, x: u16, y: u16) -> Result<&mut Cell, ()> {
        if x < self.width() && y < self.height() {
            Ok(self.get_mut_unchecked(x, y))
        } else {
            Err(())
        }
    }

    fn above<T: Layer>(&mut self, above: &T, x: u16, y: u16) {
        merge(self, above, x, y, Cell::above)
    }

    fn below<T: Layer>(&mut self, below: &T, x: u16, y: u16) {
        merge(self, below, x, y, Cell::below)
    }
}

/// Merges two layers according to function `f`
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
            if let Ok(cell_a) = a.get_mut(row, line) {
                if let Ok(cell_b) = b.get(row - x, line - y) {
                    *cell_a = f(cell_a, &cell_b);
                }
            }
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

                fn get_unchecked(&self, x: u16, _: u16) -> Cell {
                    Cell::new(
                        self.content
                            .chars()
                            .nth(x as usize)
                            .unwrap(),
                        self.style,
                    )
                }
            }
        )*
    };
}

layer_str!(String, &str,);
