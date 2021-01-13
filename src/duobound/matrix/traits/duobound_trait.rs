use crate::types::BoundMatrix;

pub trait DuoBound<T, R>: IntoIterator<Item=R> where
    R: IntoIterator<Item=T>,
{
    fn duobound(self) -> (BoundMatrix<f32>, BoundMatrix<f32>);
}



