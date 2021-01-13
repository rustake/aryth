use std::fmt;
use std::ops::Sub;

pub struct Bound<T> {
    pub min: T,
    pub max: T,
}

impl<T> Bound<T> where T: Copy + Sub<Output=T> {
    pub fn dif(&self) -> T { self.max - self.min }
}

impl<T> Bound<T> where T: Copy {
    pub fn new(min: T, max: T) -> Self { Bound { min, max } }
}

impl<T> Bound<T> where T: Default {
    pub fn default() -> Self { Bound { min: T::default(), max: T::default() } }
}

impl<T> Bound<T> where T: Copy + PartialOrd {
    pub fn expand(&mut self, v: &T)
    { if v < &self.min { self.min = *v } else if v > &self.max { self.max = *v } }
}


// impl<T> Bound<T> where T: NumOps + Copy {
//     pub fn by(min: Option<T>, max: Option<T>) -> Self {
//         let dif = match (min, max) {
//             (Some(min_v), Some(max_v)) => { Some(max_v - min_v) }
//             (_, _) => { None }
//         };
//         Bound { min, max, dif }
//     }
//     pub fn leap(min: Option<T>, dif: Option<T>) -> Self {
//         let max = match (min, dif) {
//             (Some(min_v), Some(dif_v)) => { Some(min_v + dif_v) }
//             (_, _) => { None }
//         };
//         Bound { min, max, dif }
//     }
// }

impl<T> fmt::Display for Bound<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let min = match &self.min {
        //     None => { "None".to_string() }
        //     Some(v) => { v.to_string() }
        // };
        // let max = match &self.max {
        //     None => { "None".to_string() }
        //     Some(v) => { v.to_string() }
        // };
        write!(f, "Bound {{ min: {}, max: {} }}", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bound_alpha = Bound::new(5, 7);
        // let bound_alpha = Bound::by(Some(5), Some(7));
        // let bound_beta = Bound::leap(Some(5), Some(2));
        println!("{}, dif = {}", bound_alpha, bound_alpha.dif());
        // println!("{}", bound_beta)
    }
}