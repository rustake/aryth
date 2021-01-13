pub use types::{Bound, BoundMatrix, BoundVector};

pub mod math;
pub mod duobound;
pub mod bound;
pub mod utils;

mod types;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
