use crate::types::BoundVector;

pub trait DuoBoundable<T>: IntoIterator<Item=T> {
    fn duobound(self) -> (BoundVector<f32>, BoundVector<f32>);
}