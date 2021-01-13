// use num::traits::AsPrimitive;
// use veho::vector::Mappers;
//
// use crate::types::BoundVector;
//
// pub fn duobound<'a, T>(it: impl IntoIterator<Item=&'a T>) -> (BoundVector<f32>, BoundVector<f32>) where
//     T: AsPrimitive<f32> + Copy
// {
//     let (mut min_y, mut max_y) = (None, None);
//     let vec_y = it.mapper(|x| {
//         let v = x.as_();
//         if min_y == None {
//             min_y = Some(v);
//             max_y = Some(v);
//         }
//         if v > max_y.unwrap() { max_y = Some(v) } else if v < min_y.unwrap() { min_y = Some(v) }
//         Some(v)
//     });
//     let len = (&vec_y).len();
//     return (
//         BoundVector { body: vec![None; len], min: None, max: None, count: 0 },
//         BoundVector { body: vec_y, min: min_y, max: max_y, count: len }
//     );
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {
//         let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
//         let (vec_x, vec_y) = duobound(&vec);
//         println!("{}", vec_x);
//         println!("{}", vec_y);
//         println!("{:?}", &vec);
//     }
// }