use crate::*;

pub struct Row<'a, T: ?Sized> {
    layer: &'a T,
    col:   u16,
    width: u16,
    row:   u16,
    end:   u16,
}

impl<'a, T: LayerSize + ?Sized> Row<'a, T> {
    pub fn new(layer: &'a T, col: u16, row: u16, width: u16, height: u16) -> Self {
        // layer.cropped_row will handle X-axis bound checks
        // we handle Y-axis bound checks here to avoid empty rows
        let (_, layer_height) = layer.size();
        let row = row.min(layer_height);
        let height = height.min(layer_height.saturating_sub(row));

        Self {
            layer,
            col,
            width,
            row,
            end: row + height,
        }
    }
}
