mod cell;
mod damaged_cell;

pub use cell::*;
pub use damaged_cell::*;

pub trait AsDamagedCell {
    fn get_cell(&self) -> Cell;
    fn get_damage(&self) -> Cell;

    fn save_mut(&mut self);

    fn save(mut self) -> Self
    where
        Self: Sized,
    {
        self.save_mut();
        self
    }
}
