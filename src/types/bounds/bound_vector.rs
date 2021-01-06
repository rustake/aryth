use std::fmt;

use veho::vector::Mappers;

pub struct BoundVector<T> {
    pub vec: Vec<Option<T>>,
    pub min: Option<T>,
    pub max: Option<T>,
    pub count: usize,
}

fn option_to_string<T>(v: &Option<T>) -> String where T: fmt::Display {
    match v {
        None => { "None".to_string() }
        Some(x) => { x.to_string() }
    }
}

impl<T> fmt::Display for BoundVector<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ vec: {}, min: {}, max: {}, count: {} }} ",
               format!("[ {} ]", (&self.vec).mapper(option_to_string).join(", ")),
               option_to_string(&self.min),
               option_to_string(&self.max),
               &self.count
        )
    }
}