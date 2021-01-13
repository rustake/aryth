use std::fmt;

pub fn option_to_string<T>(v: &Option<T>) -> String where T: fmt::Display {
    match v {
        None => { "None".to_string() }
        Some(x) => { x.to_string() }
    }
}