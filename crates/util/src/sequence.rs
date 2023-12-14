pub fn detect_cycle<T>(
    mut next: impl FnMut(&T) -> T,
    ne: impl Fn(&T, &T) -> bool,
    start: T,
) -> (usize, usize)
where
    T: Clone,
{
    // Brent's cycle detection algorithm.
    // Adapted from https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm

    // main phase: search successive powers of two
    let mut power = 1;
    let mut cycle_length = 1;
    let mut tortoise = start.clone();
    let mut hare = next(&start); // next(&start) is the element/node next to start.
    while ne(&tortoise, &hare) {
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
    while ne(&tortoise, &hare) {
        tortoise = next(&tortoise);
        hare = next(&hare);
        cycle_offset += 1;
    }

    (cycle_length, cycle_offset)
}
