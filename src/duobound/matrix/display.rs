use std::fmt;

use veho::vector::Mappers;

use crate::duobound::helpers::assort;
use crate::duobound::matrix::DuoBound;
use crate::types::{BoundMatrix, protract};

impl<T, R, M> DuoBound<T, R> for M where
    T: fmt::Display,
    R: IntoIterator<Item=T>,
    M: IntoIterator<Item=R>,
    M::IntoIter: Iterator<Item=R>,
{
    fn duobound(self) -> (BoundMatrix<f32>, BoundMatrix<f32>) {
        let mut bound_a = BoundMatrix { body: Vec::new(), min: None, max: None, count: 0 };
        let mut bound_b = BoundMatrix { body: Vec::new(), min: None, max: None, count: 0 };
        self.iterate(|row| {
            let mut row_a = Vec::new();
            let mut row_b = Vec::new();
            row.iterate(|element| {
                let (a, b) = assort(element);
                if let Some(v) = a { protract(&mut bound_a.min, &mut bound_a.max, &mut bound_a.count, &v) }
                if let Some(v) = b { protract(&mut bound_b.min, &mut bound_b.max, &mut bound_b.count, &v) }
                row_a.push(a);
                row_b.push(b);
            });
            bound_a.body.push(row_a);
            bound_b.body.push(row_b);
        });
        return (bound_a, bound_b);
    }
}

pub fn duobound<T, R, M>(mx: M) -> (BoundMatrix<f32>, BoundMatrix<f32>) where
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