use crate::types::VectorAndBound;

pub trait DuoBound<T>: IntoIterator<Item=T>
{ fn duobound(self) -> (VectorAndBound<f32>, VectorAndBound<f32>); }
