use std::fmt;

use veho::matrix::Matrix;
use veho::vector::Mappers;

use crate::Bound;
use crate::utils::option_to_string;

pub struct MatrixAndBound<T> (
    pub Matrix<Option<T>>,
    pub Option<Bound<T>>,
);

impl<T> MatrixAndBound<T> {
    pub fn ref_as_tuple(&self) -> (&Matrix<Option<T>>, &Option<Bound<T>>)
    { (&self.0, &self.1) }
}

impl<T> fmt::Display for MatrixAndBound<T> where
    T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (matrix, bound) = self.ref_as_tuple();
        write!(f, "{{ \n  body: {}, \n  bound: {}\n}} ",
               format!("[ \n{} \n]", matrix.mapper(|row| {
                   format!("    [ {} ],", row.mapper(option_to_string).join(", "))
               }).join("\n")),
               option_to_string(bound),
        )
    }
}

impl<T> MatrixAndBound<T> {
    // pub fn append_row(&self) -> &Vec<Option<T>> {
    //     (&self.body).push(Vec::new());
    //     return (&self.body).last().unwrap();
    // }
}

#[cfg(test)]
mod tests {
    use veho::matrix::{iso, zipper};

    use super::*;

    #[test]
    fn test() {
        let mx = [
            [Some(2), None, None, None],
            [None, Some(0), None, None],
            [None, None, Some(6), None],
            [None, None, None, Some(1)],
        ];
        let mut bound_and_matrix = MatrixAndBound(
            iso(4, 4, None),
            None,
        );
        let holder_matrix = &mut bound_and_matrix.0;
        zipper(holder_matrix, &mx, |holder, val| {
            match val {
                None => {}
                Some(v) => { *holder = Some(v) }
            }
        });
        println!("{}", bound_and_matrix);
    }
}