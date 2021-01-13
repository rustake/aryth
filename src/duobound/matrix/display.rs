use std::fmt;

use veho::entries::{MoveUnwind, RefUnwind};
use veho::vector::Mappers;

use crate::Bound;
use crate::duobound::helpers::assort;
use crate::duobound::matrix::DuoBound;
use crate::types::MatrixAndBound;

impl<T, R, M> DuoBound<T, R> for M where
    T: fmt::Display,
    R: IntoIterator<Item=T>,
    M: IntoIterator<Item=R>,
    M::IntoIter: Iterator<Item=R>,
{
    fn duobound(self) -> (MatrixAndBound<f32>, MatrixAndBound<f32>) {
        let (mut bd_x, mut bd_y) = (Bound::default(), Bound::default());
        let (mx_x, mx_y) = self.mapper(|row| {
            row.mapper(|value| {
                let (x, y) = assort(value);
                if let Some(v) = x { (&mut bd_x).expand(&v) }
                if let Some(v) = y { (&mut bd_y).expand(&v) }
                (x, y)
            }).clone_unwind()
        }).move_unwind();
        return (MatrixAndBound(mx_x, bd_x), MatrixAndBound(mx_y, bd_y));
    }
}

pub fn duobound<T, R, M>(mx: M) -> (MatrixAndBound<f32>, MatrixAndBound<f32>) where
    T: fmt::Display,
    R: IntoIterator<Item=T>,
    M: IntoIterator<Item=R>,
    M::IntoIter: Iterator<Item=R>,
{ mx.duobound() }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let vec = vec![
            vec!["1", "2", "3"],
            vec!["a", "b", "c"],
            vec!["-", "-", "-"],
        ];
        let (vec_x, vec_y) = (&vec).duobound();
        println!("{}", vec_x);
        println!("{}", vec_y);
        println!("original {:?}", vec);
    }

    // #[test]
    // fn test_arr() {
    //     let vec = ["1", "2", "3", "a", "b", "c"];
    //     let (vec_x, vec_y) = duobound(&vec);
    //     println!("{}", vec_x);
    //     println!("{}", vec_y);
    //     println!("{:?}", vec);
    // }
}