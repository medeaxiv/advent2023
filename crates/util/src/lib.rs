pub fn greatest_common_divisor<T>(mut a: T, mut b: T) -> T
where
    T: num::Integer + Clone,
{
    while b != T::zero() {
        let t = b.clone();
        b = a % b;
        a = t;
    }

    a
}

pub fn least_common_multiple<T>(a: T, b: T) -> T
where
    T: num::Integer + Clone,
{
    if a == T::zero() && b == T::zero() {
        return T::zero();
    }

    let gcd = greatest_common_divisor(a.clone(), b.clone());
    (a * b) / gcd
}
