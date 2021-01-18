use veho::vector::Reduces;

pub trait Indicators: IntoIterator {
    fn max_by<T, F>(self, indicator: F) -> Option<T> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=Self::Item>,
        F: Fn(Self::Item) -> T,
        T: Ord,
    { self.into_iter().mapreduce(indicator, Ord::max) }

    fn min_by<T, F>(self, indicator: F) -> Option<T> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=Self::Item>,
        F: Fn(Self::Item) -> T,
        T: Ord,
    { self.into_iter().mapreduce(indicator, Ord::min) }
}

impl<I> Indicators for I where
    I: IntoIterator + ?Sized
{}

pub fn max_by<I, T, F>(vec: I, indicator: F) -> Option<T> where
    I: IntoIterator + Sized,
    I::IntoIter: Iterator<Item=I::Item>,
    F: Fn(I::Item) -> T,
    T: Ord,
{ vec.max_by(indicator) }

pub fn min_by<I, T, F>(vec: I, indicator: F) -> Option<T> where
    I: IntoIterator + Sized,
    I::IntoIter: Iterator<Item=I::Item>,
    F: Fn(I::Item) -> T,
    T: Ord,
{ vec.min_by(indicator) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_alpha() {
        let vec = vec!["fo", "z", "bar", "shakes"];
        let k = "some";
        // let bounded = &vec.max_by();
        let max_v = (&vec).max_by(|x| x.len());
        println!("{} max: {}", k, max_v.unwrap());
        println!("{} original: {:?}", k, vec);
    }

    // #[test]
    // fn test_ref_beta() {
    //     let candidates = vec![
    //         ("empty", vec![]),
    //         ("sole", vec![1]),
    //         ("some", vec![4, 5, 9, 3, 7, 1])
    //     ].into_hashmap();
    //     (&candidates).iterate(|k, vec| {
    //         let bounded = bound(vec);
    //         println!("{} bound: {}", k, option_to_string(&bounded));
    //         println!("{} original: {:?}", k, vec);
    //     });
    //     println!("original candidates = {:?}", candidates);
    // }
}