use nalgebra::{ClosedAdd, ClosedSub, Dim, Scalar, StorageMut, Vector};
use num::Zero;

pub fn manhattan_distance<T, D, S>(a: Vector<T, D, S>, b: Vector<T, D, S>) -> T
where
    T: PartialOrd + std::iter::Sum + Scalar + Zero + ClosedAdd + ClosedSub,
    D: Dim,
    S: StorageMut<T, D>,
{
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| {
            if b > a {
                b.clone() - a.clone()
            } else {
                a.clone() - b.clone()
            }
        })
        .sum()
}
