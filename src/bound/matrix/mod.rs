use veho::matrix::iterate;

use crate::types::Bound;

pub fn bound<M, R>(mx: M) -> Option<Bound<R::Item>> where
    M: IntoIterator<Item=R>,
    M::IntoIter: Iterator<Item=R>,
    R: IntoIterator,
    R::Item: Copy + PartialOrd,
    R::IntoIter: Iterator<Item=R::Item>

{
    let rows_iter = &mut mx.into_iter();
    match (rows_iter).next() {
        None => { None }
        Some(row) => {
            match (&mut row.into_iter()).next() {
                None => { None }
                Some(val) => {
                    let (mut min, mut max) = (*&val, *&val);
                    iterate(rows_iter, |x| { if x > max { max = x } else if x < min { min = x } });
                    Some(Bound::new(min, max))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use veho::hashmap::{Mappers};

    use super::*;
    use veho::entries::IntoHashmap;

    #[test]
    fn test() {
        let candidates = vec![
            ("empty", vec![]),
            ("row", vec![
                vec![1, 2, 3]
            ]),
            ("wierd_row", vec![
                vec![],
                vec![1, 2, 3]
            ]),
            ("column", vec![
                vec![1, ],
                vec![4, ],
                vec![7, ],
            ]),
            ("some", vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
            ])
        ].into_hashmap();
        candidates.iterate(|k, mx| {
            let bounded = bound(&mx);
            println!("{}: {}", k, bounded.unwrap());
        });
    }
}