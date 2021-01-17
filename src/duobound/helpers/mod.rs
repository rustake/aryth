use std::fmt;

use texting::str_value;

use crate::Bound;
use crate::types::expand_opt_bound;

pub fn assort<T>(x: T) -> (Option<f32>, Option<f32>) where
    T: fmt::Display
{
    let tx = x.to_string();
    match tx.parse::<f32>() {
        Err(_) => { (Some(str_value(&tx) as f32), None) }
        Ok(n) => { (None, Some(n)) }
    }
}

pub fn assort_expand_entry_bound<T>(bound_x: &mut Option<Bound<f32>>, bound_y: &mut Option<Bound<f32>>, value: &T) -> (Option<f32>, Option<f32>) where
    T: fmt::Display
{
    let (x, y) = assort(&value);
    expand_opt_bound(bound_x, &x);
    expand_opt_bound(bound_y, &y);
    (x, y)
}