pub fn greatest_common_divisor<T>(mut a: T, mut b: T) -> T
where
    T: num_traits::PrimInt,
{
    while b != T::zero() {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

pub fn least_common_multiple<T>(a: T, b: T) -> T
where
    T: num_traits::PrimInt,
{
    if a == T::zero() && b == T::zero() {
        return T::zero();
    }

    (a * b) / greatest_common_divisor(a, b)
}
