use num::Num;
use num::traits::AsPrimitive;

fn int_exp<T>(x: T) -> i32 where T: Num + AsPrimitive<f32> + Copy {
    x.as_().abs().log10() as i32
}

// fn int_exp<T>(x: T) -> T where T: Num + AsPrimitive<f32> + FromPrimitive + Copy {
//     let v = x.as_();
//     println!("{}", v);
//     let result: Option<T> = FromPrimitive::from_f32(v.abs().log10().floor());
//     match result {
//         None => { x }
//         Some(v) => { v }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = 720;
        let result = int_exp(x);
        println!("{}", result);
    }
}