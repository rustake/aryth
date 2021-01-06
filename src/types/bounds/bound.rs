use std::fmt;

use num::traits::NumOps;

pub struct Bound<T> {
    pub min: Option<T>,
    pub max: Option<T>,
    pub dif: Option<T>,
}

impl<T> Bound<T> where T: Copy {
    pub fn new(min: Option<T>, max: Option<T>) -> Self {
        Bound { min, max, dif: None }
    }
}

impl<T> Bound<T> where T: NumOps + Copy {
    pub fn by(min: Option<T>, max: Option<T>) -> Self {
        let dif = match (min, max) {
            (Some(min_v), Some(max_v)) => { Some(max_v - min_v) }
            (_, _) => { None }
        };
        Bound { min, max, dif }
    }
    pub fn leap(min: Option<T>, dif: Option<T>) -> Self {
        let max = match (min, dif) {
            (Some(min_v), Some(dif_v)) => { Some(min_v + dif_v) }
            (_, _) => { None }
        };
        Bound { min, max, dif }
    }
}

impl<T> fmt::Display for Bound<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min = match &self.min {
            None => { "None".to_string() }
            Some(v) => { v.to_string() }
        };
        let max = match &self.max {
            None => { "None".to_string() }
            Some(v) => { v.to_string() }
        };
        write!(f, "Bound {{ min: {}, max: {} }}", min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bound_alpha = Bound::by(Some(5), Some(7));
        let bound_beta = Bound::leap(Some(5), Some(2));
        println!("{}", bound_alpha);
        println!("{}", bound_beta)
    }
}