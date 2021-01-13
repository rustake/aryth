use std::fmt;

use veho::vector::Mappers;

use crate::duobound::helpers::assort;
use crate::duobound::vector::DuoBound;
use crate::types::BoundVector;

impl<IT, T> DuoBound<T> for IT where
    T: fmt::Display,
    IT: IntoIterator<Item=T>,
    IT::IntoIter: Iterator<Item=T>,
{
    fn duobound(self) -> (BoundVector<f32>, BoundVector<f32>) {
        let mut bound_a = BoundVector { body: Vec::new(), min: None, max: None, count: 0 };
        let mut bound_b = BoundVector { body: Vec::new(), min: None, max: None, count: 0 };
        self.iterate(|x| {
            let (a, b) = assort(x);
            bound_a.push(a);
            bound_b.push(b);
        });
        return (bound_a, bound_b);
    }
}

pub fn duobound<I>(it: I) -> (BoundVector<f32>, BoundVector<f32>) where
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
        println!("{:?}", vec);
    }
}