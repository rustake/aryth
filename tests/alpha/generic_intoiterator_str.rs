use std::fmt;

use texting::str_value;
use veho::vector::{mapper, Mappers};

pub fn iter_str_value<IT>(it: IT) -> Vec<u32> where
    IT: IntoIterator,
    IT::Item: fmt::Display,
{
    return it.mapper(|x| str_value(&x.to_string()));
}

#[test]
fn test() {
    let vec = vec!["1", "2", "3"];
    let result = iter_str_value(&vec);
    // println!("{:?}", result);
    println!("{:?}", vec);
}