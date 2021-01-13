use crate::types::BoundVector;

pub trait DuoBound<T>: IntoIterator<Item=T>
{
    fn duobound(self) -> (BoundVector<f32>, BoundVector<f32>);
}
