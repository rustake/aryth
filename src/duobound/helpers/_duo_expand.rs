use std::fmt;

use crate::Bound;
use crate::duobound::helpers::assort;
use crate::types::expand_opt_bound;

pub fn assort_expand_entry_bound<T>(bound_x: &mut Option<Bound<f32>>, bound_y: &mut Option<Bound<f32>>, value: &T) -> (Option<f32>, Option<f32>) where
    T: fmt::Display
{
    let (x, y) = assort(&value);
    expand_opt_bound(bound_x, &x);
    expand_opt_bound(bound_y, &y);
    (x, y)
}