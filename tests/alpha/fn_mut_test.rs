use std::fmt;

pub fn option_to_string<T>(v: &Option<T>) -> String where T: fmt::Display {
    match v {
        None => { "None".to_string() }
        Some(x) => { x.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use veho::vector::Mappers;

    use super::*;

    #[test]
    fn fn_mut_test() {
        let mut min = None;
        let mut max = None;
        let mut count = 0;
        let vec = [1, 2, 3, 4, 5];
        fn protract(min: &mut Option<i32>, max: &mut Option<i32>, count: &mut i32, v: &i32) {
            *count += 1;
            match (&min, &max) {
                (Some(vi), Some(va)) => {
                    if v < vi { *min = Some(*v) } else if v > va { *max = Some(*v) }
                }
                (_, _) => {
                    *min = Some(*v);
                    *max = Some(*v);
                }
            }
        }
        (&vec).iterate(|x| {
            protract(&mut min, &mut max, &mut count, x);
        });
        println!("#[{}] min = {}, max = {}",
                 &count,
                 option_to_string(&min),
                 option_to_string(&max)
        );
    }
}