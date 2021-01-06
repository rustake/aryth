use veho::vector::iterate;

use crate::types::Bound;

pub fn bound<I>(it: I) -> Bound<I::Item> where
    I: IntoIterator,
    I::Item: Copy + PartialOrd,
    I::IntoIter: Iterator<Item=I::Item>
{
    let iter = &mut it.into_iter();
    match iter.next() {
        None => { Bound::new(None, None) }
        Some(first) => {
            let (mut min, mut max) = (*&first, *&first);
            iterate(iter, |x| { if x > max { max = x } else if x < min { min = x } });
            Bound::new(Some(min), Some(max))
        }
    }
}

#[cfg(test)]
mod tests {
    use veho::hashmap::{Init, Mappers};

    use super::*;

    #[test]
    fn test() {
        let candidates = vec![
            ("empty", vec![]),
            ("sole", vec![1]),
            ("some", vec![4, 5, 9, 3, 7, 1])
        ].into_hashmap();
        (&candidates).iterate(|k, v| {
            let bounded = bound(v);
            println!("{}: {}", k, bounded);
        });
        println!("original candidates = {:?}", candidates);
    }
}