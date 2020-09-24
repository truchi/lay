use super::Cell;

/// A rectangle of `Cell`s.
#[derive(Eq, PartialEq, Default, Debug)]
pub struct Canvas {
    cells:  Vec<Cell>,
    width:  u16,
    height: u16,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self::with_cell(Cell::default(), width, height)
    }

    pub fn with_cell(cell: Cell, width: u16, height: u16) -> Self {
        let size = width * height;
        let mut cells = Vec::with_capacity(usize::from(size));
        cells.resize(usize::from(size), cell);

        Self {
            cells,
            width,
            height,
        }
    }
}

impl From<(u16, u16)> for Canvas {
    fn from((width, height): (u16, u16)) -> Self {
        Self::new(width, height)
    }
}

impl_layer!(Canvas [canvas, x, y] {
    Layer { canvas.width } { canvas.height } { canvas[(x, y)] }
    Index {
        canvas.cells
            .get(usize::from(x) + usize::from(y) * usize::from(canvas.width))
            .unwrap()
    }
    LayerMut { &mut canvas[(x, y)] }
    IndexMut {
        canvas.cells
            .get_mut(usize::from(x) + usize::from(y) * usize::from(canvas.width))
            .unwrap()
    }
});

impl_layer_mut_ops!(Canvas);
