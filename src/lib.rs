#[macro_use]
mod macros;

#[macro_use]
mod test;
pub use test::*;
mod style;
pub use style::*;

// mod layer;
// mod style;
//
// pub use layer::*;
// pub use style::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
