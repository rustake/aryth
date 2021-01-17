use veho::matrix::iterate;

use crate::indicator::vector::{
    max_by as vector_max_by,
    min_by as vector_min_by,
};

pub trait Indicators<R>: IntoIterator<Item=R> where
    R: IntoIterator,
{
    fn max_by<T, F>(self, indicator: F) -> Option<T> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=R>,
        R::Item: Ord,
        R::IntoIter: Iterator<Item=R::Item>,
        F: Fn(R::Item) -> T,
        T: Ord,
    {
        let rows_iter = &mut self.into_iter();
        match (rows_iter).next() {
            None => { None }
            Some(row) => {
                let max = vector_max_by(row, &indicator);
                match max {
                    None => { None }
                    Some(mut a) => {
                        iterate(rows_iter, |x| {
                            let v = indicator(x);
                            if v > a { a = v }
                        });
                        Some(a)
                    }
                }
            }
        }
    }

    fn min_by<T, F>(self, indicator: F) -> Option<T> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=R>,
        R::Item: Ord,
        R::IntoIter: Iterator<Item=R::Item>,
        F: Fn(R::Item) -> T,
        T: Ord,
    {
        let rows_iter = &mut self.into_iter();
        match (rows_iter).next() {
            None => { None }
            Some(row) => {
                let min = vector_min_by(row, &indicator);
                match min {
                    None => { None }
                    Some(mut a) => {
                        iterate(rows_iter, |x| {
                            let v = indicator(x);
                            if v < a { a = v }
                        });
                        Some(a)
                    }
                }
            }
        }
    }
}

impl<R, M> Indicators<R> for M where
    M: IntoIterator<Item=R>,
    R: IntoIterator
{}

// fn max_by<I, T, F>(vec: I, indicator: F) -> Option<T> where
//     I: IntoIterator + Sized,
//     I::IntoIter: Iterator<Item=I::Item>,
//     F: Fn(I::Item) -> T,
//     T: Ord,
// { vec.max_by(indicator) }
//
// fn min_by<I, T, F>(vec: I, indicator: F) -> Option<T> where
//     I: IntoIterator + Sized,
//     I::IntoIter: Iterator<Item=I::Item>,
//     F: Fn(I::Item) -> T,
//     T: Ord,
// { vec.min_by(indicator) }

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
                vec!["f", "ba", "zen"]
            ]),
            ("wierd_row", vec![
                vec![],
                vec!["f", "ba", "zen"]
            ]),
            ("column", vec![
                vec!["f", ],
                vec!["ba", ],
                vec!["zen", ],
            ]),
            ("some", vec![
                vec!["fo", "bar", "zenone"],
                vec!["foo", "bare", "zene"],
                vec!["f", "ba", "zen"],
            ])
        ].into_hashmap();
        candidates.iterate(|k, mx| {
            let bounded = (&mx).max_by(|x| x.len());
            println!("{}: {}", k, option_to_string(&bounded));
        });
    }
}