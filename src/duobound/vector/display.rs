use std::fmt;

use veho::entries::MoveUnwind;
use veho::vector::Mappers;

use crate::Bound;
use crate::duobound::helpers::assort;
use crate::duobound::vector::DuoBound;
use crate::types::VectorAndBound;

impl<IT, T> DuoBound<T> for IT where
    T: fmt::Display,
    IT: IntoIterator<Item=T>,
    IT::IntoIter: Iterator<Item=T>,
{
    fn duobound(self) -> (VectorAndBound<f32>, VectorAndBound<f32>) {
        let (mut bd_x, mut bd_y) = (Bound::default(), Bound::default());
        let (ve_x, ve_y) = self
            .mapper(|value| {
                let (x, y) = assort(value);
                if let Some(v) = x { (&mut bd_x).expand(&v) }
                if let Some(v) = y { (&mut bd_y).expand(&v) }
                (x, y)
            }).move_unwind();
        return (VectorAndBound(ve_x, bd_x), VectorAndBound(ve_y, bd_y));
    }
}

pub fn duobound<I>(it: I) -> (VectorAndBound<f32>, VectorAndBound<f32>) where
    I: IntoIterator,
    I::Item: fmt::Display,
{ it.duobound() }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let vec = vec!["1", "2", "3", "a", "b", "c"];
        let (vec_x, vec_y) = (&vec).duobound();
        println!("{}", vec_x);
        println!("{}", vec_y);
        println!("original {:?}", vec);
    }

    #[test]
    fn test_arr() {
        let vec = ["1", "2", "3", "a", "b", "c"];
        let (vec_x, vec_y) = duobound(&vec);
        println!("{}", vec_x);
        println!("{}", vec_y);
        println!("original {:?}", vec);
    }
}