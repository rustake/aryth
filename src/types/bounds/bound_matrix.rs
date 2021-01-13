use std::fmt;

use veho::matrix::Matrix;
use veho::vector::Mappers;

use crate::utils::option_to_string;

pub struct BoundMatrix<T> {
    pub body: Matrix<Option<T>>,
    pub min: Option<T>,
    pub max: Option<T>,
    pub count: usize,
}

impl<T> fmt::Display for BoundMatrix<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \n  body: {}, \n  min: {}, \n  max: {}, \n  count: {} \n}} ",
               format!("[ \n{} \n]", (&self.body).mapper(|row| {
                   format!("    [ {} ],", row.mapper(option_to_string).join(", "))
               }).join("\n")),
               option_to_string(&self.min),
               option_to_string(&self.max),
               &self.count
        )
    }
}

impl<T> BoundMatrix<T> {
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
        let mut bound_matrix = BoundMatrix {
            body: iso(4, 4, None),
            min: None,
            max: None,
            count: 0,
        };
        let holder_matrix = &mut bound_matrix.body;
        zipper(holder_matrix, &mx, |holder, val| {
            match val {
                None => {}
                Some(v) => { *holder = Some(v) }
            }
        });
        println!("{}", bound_matrix);
    }
}