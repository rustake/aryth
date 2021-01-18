use veho::matrix::Reduces;

use crate::types::Bound;

pub fn bound<M, R>(mx: M) -> Option<Bound<R::Item>> where
    M: IntoIterator<Item=R>,
    M::IntoIter: Iterator<Item=R>,
    R: IntoIterator,
    R::Item: Copy + PartialOrd,
    R::IntoIter: Iterator<Item=R::Item>
{
    mx.mapflat(|x| Bound::new(x, x),
               |mut b, x| {
                   if x > b.max { b.max = x } else if x < b.min { b.min = x }
                   b
               })
}

#[cfg(test)]
mod tests {
    use veho::entries::IntoHashmap;
    use veho::hashmap::Mappers;

    use crate::utils::option_to_string;

    use super::*;

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
                vec![1, 3, 9],
                vec![1, 2, 4],
                vec![1, 1, 1],
            ])
        ].into_hashmap();
        candidates.iterate(|k, mx| {
            let bounded = bound(&mx);
            println!("{}: {}", k, option_to_string(&bounded));
        });
    }
}