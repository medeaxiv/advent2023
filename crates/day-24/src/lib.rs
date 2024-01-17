use std::ops::RangeBounds;

use itertools::Itertools;
use nalgebra::{vector, Matrix3, Vector2, Vector3};

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input, 200000000000000..=400000000000000)
}

fn solve_part1<R>(input: &str, bounds: R) -> usize
where
    R: RangeBounds<i64>,
{
    input
        .lines()
        .map(parse_int)
        .tuple_combinations()
        .filter(|(a, b)| intersects_in_bounds(a, b, &bounds))
        .count()
}

fn intersects_in_bounds<R>(a: &Hailstone<i64>, b: &Hailstone<i64>, bounds: &R) -> bool
where
    R: RangeBounds<i64>,
{
    let va = a.velocity.xy();
    let pa1 = a.position.xy();
    let pa2 = pa1 + va;

    let vb = b.velocity.xy();
    let pb1 = b.position.xy();
    let pb2 = pb1 + vb;

    let Some(i) = intersection_point(pa1, pa2, pb1, pb2) else {
        // No intersection point
        return false;
    };

    if (i.x - pa1.x).signum() != va.x.signum() || (i.x - pb1.x).signum() != vb.x.signum() {
        // Intersection point in the past
        return false;
    }

    bounds.contains(&i.x) && bounds.contains(&i.y)
}

fn intersection_point(
    a1: Vector2<i64>,
    a2: Vector2<i64>,
    b1: Vector2<i64>,
    b2: Vector2<i64>,
) -> Option<Vector2<i64>> {
    let (x1, x2, x3, x4) = (a1.x as i128, a2.x as i128, b1.x as i128, b2.x as i128);
    let (y1, y2, y3, y4) = (a1.y as i128, a2.y as i128, b1.y as i128, b2.y as i128);

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if denom == 0 {
        return None;
    }

    let a_det = x1 * y2 - y1 * x2;
    let b_det = x3 * y4 - y3 * x4;

    // Integer division
    // The coordinates are not exact
    // This may cause issues with the bounds check
    // It does not cause issues with my input, or the test input
    let x = (a_det * (x3 - x4) - (x1 - x2) * b_det) / denom;
    let y = (a_det * (y3 - y4) - (y1 - y2) * b_det) / denom;

    Some(vector![x as i64, y as i64])
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> i64 {
    let result = input
        .lines()
        .map(parse_float)
        .tuple_combinations()
        .filter_map(|(a, b, c)| {
            let m = Matrix3::from_rows(&[
                (a.velocity - b.velocity)
                    .cross(&(a.position - b.position))
                    .transpose(),
                (a.velocity - c.velocity)
                    .cross(&(a.position - c.position))
                    .transpose(),
                (b.velocity - c.velocity)
                    .cross(&(b.position - c.position))
                    .transpose(),
            ]);

            let inv_m = m.try_inverse()?;

            let d = vector![
                (a.velocity - b.velocity).dot(&a.position.cross(&b.position)),
                (a.velocity - c.velocity).dot(&a.position.cross(&c.position)),
                (b.velocity - c.velocity).dot(&b.position.cross(&c.position)),
            ];

            Some(inv_m * d)
        })
        .next()
        .unwrap();

    // I got lucky and f64 had enough precision for this to be correct with my input
    (result.x + result.y + result.z) as i64
}

#[derive(Debug, Clone, Copy)]
struct Hailstone<T> {
    position: Vector3<T>,
    velocity: Vector3<T>,
}

impl<T> Hailstone<T> {
    pub fn new(position: Vector3<T>, velocity: Vector3<T>) -> Self {
        Self { position, velocity }
    }
}

fn parse_int(input: &str) -> Hailstone<i64> {
    fn parse_vector(input: &str) -> Vector3<i64> {
        let (x, y, z) = input
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .next_tuple()
            .unwrap();
        vector![x, y, z]
    }

    let (position, velocity) = input.split_once('@').unwrap();
    Hailstone::new(parse_vector(position), parse_vector(velocity))
}

fn parse_float(input: &str) -> Hailstone<f64> {
    fn parse_vector(input: &str) -> Vector3<f64> {
        let (x, y, z) = input
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap())
            .next_tuple()
            .unwrap();
        vector![x, y, z]
    }

    let (position, velocity) = input.split_once('@').unwrap();
    Hailstone::new(parse_vector(position), parse_vector(velocity))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT, 7..=27);
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 47);
    }
}
