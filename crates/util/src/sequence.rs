use crate::cache::Cache;

pub fn detect_cycle<T>(mut next: impl FnMut(&T) -> T, start: T) -> (usize, usize)
where
    T: Clone + PartialEq,
{
    // Brent's cycle detection algorithm.
    // Adapted from https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm

    // main phase: search successive powers of two
    let mut power = 1;
    let mut cycle_length = 1;
    let mut tortoise = start.clone();
    let mut hare = next(&start); // next(&start) is the element/node next to start.
    while tortoise != hare {
        if power == cycle_length {
            // time to start a new power of two?
            tortoise = hare.clone();
            power *= 2;
            cycle_length = 0;
        }

        hare = next(&hare);
        cycle_length += 1;
    }

    // Find the position of the first repetition of length λ (cycle_length)
    tortoise = start.clone();
    hare = start;
    for _ in 0..cycle_length {
        // 0..cycle_length produces a list with the values 0, 1, ... , cycle_length-1
        hare = next(&hare);
    }
    // The distance between the hare and tortoise is now λ (cycle_length).

    // Next, the hare and tortoise move at same speed until they agree
    let mut cycle_offset = 0;
    while tortoise != hare {
        tortoise = next(&tortoise);
        hare = next(&hare);
        cycle_offset += 1;
    }

    (cycle_length, cycle_offset)
}

pub fn detect_cycle_cached<T, C>(mut next: impl FnMut() -> T, cache: &mut C) -> (usize, usize)
where
    C: Cache<T, usize>,
{
    let cycle_offset;
    let mut counter = 0;

    loop {
        let entry = next();

        if let Some(&idx) = cache.get(&entry) {
            cycle_offset = idx;
            break;
        }

        cache.insert(entry, counter);
        counter += 1;
    }

    let cycle_length = counter - cycle_offset;
    (cycle_length, cycle_offset)
}
