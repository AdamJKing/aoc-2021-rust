pub fn count_increases(readings: impl IntoIterator<Item = i32>) -> i32 {
    let (total, _) = readings
        .into_iter()
        .fold((0, None), |(count, previous), reading| match previous {
            None => (0, Some(reading)),
            Some(p) if p < reading => (count + 1, Some(reading)),
            Some(_) => (count, Some(reading)),
        });
    total
}
