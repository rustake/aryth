use std::fmt;

use texting::str_value;

pub fn assort<T>(x: T) -> (Option<f32>, Option<f32>) where
    T: fmt::Display
{
    let tx = x.to_string();
    match tx.parse::<f32>() {
        Err(_) => { (Some(str_value(&tx) as f32), None) }
        Ok(n) => { (None, Some(n)) }
    }
}