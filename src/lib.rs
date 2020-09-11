#[macro_use]
mod macros;

mod layer;
mod style;

pub use layer::*;
pub use style::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
