// use std::fmt;
//
// use veho::vector::Mappers;
//
// use crate::utils::option_to_string;
//
// pub struct BoundVector<T> {
//     pub body: Vec<Option<T>>,
//     pub min: Option<T>,
//     pub max: Option<T>,
//     pub count: usize,
// }
//
// impl<T> BoundVector<T> where T: Copy + PartialOrd {
//     pub fn push(&mut self, element: Option<T>) {
//         match &element {
//             Some(v) => {
//                 protract(&mut self.min, &mut self.max, &mut self.count, v);
//                 self.body.push(Some(*v));
//             }
//             None => {
//                 self.body.push(None)
//             }
//         }
//     }
// }
//
// impl<T> fmt::Display for BoundVector<T> where T: fmt::Display {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{{ body: {}, min: {}, max: {}, count: {} }} ",
//                format!("[ {} ]", (&self.body).mapper(option_to_string).join(", ")),
//                option_to_string(&self.min),
//                option_to_string(&self.max),
//                &self.count
//         )
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use veho::vector::Mappers;
//
//     use super::*;
//
//     #[test]
//     fn test() {
//         let arr = [Some(3), None, None, Some(1), Some(0), Some(6)];
//         let mut bound_vector = BoundVector {
//             body: Vec::new(),
//             min: None,
//             max: None,
//             count: 0,
//         };
//         (&arr).iterate(|x| {
//             bound_vector.push(*x);
//         });
//         println!("{}", bound_vector);
//     }
// }