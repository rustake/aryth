// use crate::Bound;
//
// pub trait Protract<T> {
//     fn protract(&mut self, value: Option<T>);
//     // {
//     //     if let Some(v) = value {
//     //         if v < self.min { self.min = v }
//     //     }
//     // }
// }
//
//
// pub fn protract<T>(min: &mut Option<T>, max: &mut Option<T>, count: &mut usize, v: &T) where
//     T: PartialOrd + Copy
// {
//     *count += 1;
//     match (&min, &max) {
//         (Some(vi), Some(va)) => {
//             if v < vi { *min = Some(*v) } else if v > va { *max = Some(*v) }
//         }
//         (_, _) => {
//             *min = Some(*v);
//             *max = Some(*v);
//         }
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
//     fn test_expand_bound() {
//         let mut bound = Some(Bound::new(0, 0));
//         let vec = vec![-2, 0, 2, 4, 6];
//         vec.iterate(|x| {
//             expand_bound(&mut bound, &Some(x));
//         });
//         println!("{}", bound.unwrap());
//     }
// }