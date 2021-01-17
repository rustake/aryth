use crate::types::MatrixAndBound;

pub trait DuoBound<T, R>: IntoIterator<Item=R> where
    R: IntoIterator<Item=T>,
{ fn duobound(self) -> (MatrixAndBound<f32>, MatrixAndBound<f32>); }



