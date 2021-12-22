pub fn count_increases(readings: &[i32], window: usize) -> i32 {
    let (total, _) = readings.windows(window).map(|w| w.iter().sum()).fold(
        (0, None::<i32>),
        |(count, previous), reading| match previous {
            None => (0, Some(reading)),
            Some(p) if p < reading => (count + 1, Some(reading)),
            Some(_) => (count, Some(reading)),
        },
    );
    total
}
