use std::fmt;

use veho::vector::Mappers;

use crate::Bound;
use crate::utils::option_to_string;

// pub type VecAndBound<T> = (Vec<Option<T>>, Option<Bound<T>>);

pub struct VectorAndBound<T>(pub Vec<Option<T>>, pub Option<Bound<T>>);

impl<T> VectorAndBound<T> {
    pub fn ref_as_tuple(&self) -> (&Vec<Option<T>>, &Option<Bound<T>>)
    { (&self.0, &self.1) }
}

impl<T> fmt::Display for VectorAndBound<T> where
    T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (vec, bound) = self.ref_as_tuple();
        write!(f, "( vec: {}, bound: {} )",
               format!("[ {} ]", vec.mapper(|x| option_to_string(x)).join(", ")),
               option_to_string(bound),
        )
    }
}