use crate::Bound;

pub fn expand_bound<T>(bound: &mut Option<Bound<T>>, value: &Option<T>) where
    T: PartialOrd + Copy
{
    if let Some(v) = value {
        match bound {
            Some(b) => { b.expand(v) }
            None => { *bound = Some(Bound::new(*v, *v)); }
        }
    }
}