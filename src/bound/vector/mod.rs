use veho::vector::iterate;

use crate::types::Bound;

pub fn bound<I>(it: I) -> Option<Bound<I::Item>> where
    I: IntoIterator,
    I::Item: Copy + PartialOrd,
    I::IntoIter: Iterator<Item=I::Item>
{
    let iter = &mut it.into_iter();
    match iter.next() {
        None => { None }
        Some(first) => {
            let (mut min, mut max) = (*&first, *&first);
            iterate(iter, |x| { if x > max { max = x } else if x < min { min = x } });
            Some(Bound::new(min, max))
        }
    }
}

#[cfg(test)]
mod tests {
    use veho::hashmap::{Mappers, MoveInit};

    use super::*;

    #[test]
    fn test_ref_alpha() {
        let vec = vec![4, 5, 9, 3, 7, 1];
        let k = "some";
        let bounded = bound(&vec);
        println!("{} bound: {}", k, bounded.unwrap());
        println!("{} original: {:?}", k, vec);
    }

    #[test]
    fn test_ref_beta() {
        let candidates = vec![
            ("empty", vec![]),
            ("sole", vec![1]),
            ("some", vec![4, 5, 9, 3, 7, 1])
        ].into_hashmap();
        (&candidates).iterate(|k, vec| {
            let bounded = bound(vec).unwrap();
            println!("{} bound: {}", k, bounded);
            println!("{} original: {:?}", k, vec);
        });
        println!("original candidates = {:?}", candidates);
    }
}