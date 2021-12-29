pub fn gamma_and_epsilon_rates<const S: usize>(bits: &[[bool; S]]) -> (u32, u32) {
    let gamma_rate = build_binary(most_common_bits(bits.iter()));
    let epsilon_rate = build_binary(least_common_bits(bits.iter()));

    (gamma_rate, epsilon_rate)
}

pub fn oxygen_generator_rating<const S: usize>(bits: &[[bool; S]]) -> Option<u32> {
    iterative_search(|x| most_common_bits(x), bits).map(build_binary)
}

pub fn co2_scrubber_rating<const S: usize>(bits: &[[bool; S]]) -> Option<u32> {
    iterative_search(|x| least_common_bits(x), bits).map(build_binary)
}

fn iterative_search<T, F, const S: usize>(comb_func: F, coll: &[[T; S]]) -> Option<[T; S]>
where
    T: Clone + PartialEq,
    F: for<'b> Fn(Box<dyn Iterator<Item = &'b [T; S]> + 'b>) -> [T; S],
{
    let mut search = coll.to_vec();
    let mut pos = 0;
    let mut comb = comb_func(Box::new(search.iter()));

    while search.len() > 1 && pos <= comb.len() {
        search.retain(|row| comb[pos] == row[pos]);
        pos += 1;
        comb = comb_func(Box::new(search.iter()));
    }

    search.get(0).cloned()
}

pub fn as_binary_rows<const S: usize>(input: impl IntoIterator<Item = String>) -> Vec<[bool; S]> {
    input
        .into_iter()
        .map(|s| {
            let mut row: [bool; S] = [false; S];
            let nums: Vec<bool> = s.as_bytes().iter().map(|&x| x == b'1').collect();
            row.copy_from_slice(&nums);
            row
        })
        .collect()
}

fn build_binary<const S: usize>(xs: [bool; S]) -> u32 {
    let mut bin = 0;
    for (i, is_bit) in xs.iter().rev().enumerate() {
        if *is_bit {
            bin += u32::pow(2, i as u32);
        }
    }

    bin
}

fn most_common_bits<'a, const S: usize>(bits: impl Iterator<Item = &'a [bool; S]>) -> [bool; S] {
    let mut total = 0;
    let mut counts = [0; S];

    for row in bits {
        for i in 0..S {
            if row[i] {
                counts[i] += 1;
            }
        }
        total += 1;
    }

    counts.map(|c| c >= (total - c))
}

#[allow(clippy::overflow_check_conditional)]
fn least_common_bits<'a, const S: usize>(bits: impl Iterator<Item = &'a [bool; S]>) -> [bool; S] {
    let mut total = 0;
    let mut counts = [0; S];

    for row in bits {
        for i in 0..S {
            if row[i] {
                counts[i] += 1;
            }
        }
        total += 1;
    }

    counts.map(|c| c < (total - c))
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn should_calculate_gamma_properly() {
        let input: Vec<[bool; 5]> = as_binary_rows([
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ]);

        let (gamma, _) = gamma_and_epsilon_rates(&input);
        assert_eq!(gamma, 22);
    }

    #[test]
    fn should_calculate_epsilon_properly() {
        let input: Vec<[bool; 5]> = as_binary_rows([
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ]);

        let (_, epsilon) = gamma_and_epsilon_rates(&input);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn should_build_binary() {
        let result = build_binary([true, false, true, true, false, true]);
        assert_eq!(result, 0b101101);
    }

    #[test]
    fn test_common_bits() {
        let input = as_binary_rows([
            "001".into(),
            "110".into(),
            "110".into(),
            "110".into(),
            "100".into(),
        ]);

        assert_eq!(
            least_common_bits(input.iter()),
            [false, false, true],
            "least common"
        );
        assert_eq!(
            most_common_bits(input.iter()),
            [true, true, false],
            "most common"
        );
    }

    #[test]
    fn test_common_bits_when_equal() {
        let input: Vec<[bool; 2]> =
            as_binary_rows(["00".into(), "11".into(), "00".into(), "11".into()]);

        assert_eq!(most_common_bits(input.iter()), [true, true], "most common");
        assert_eq!(
            least_common_bits(input.iter()),
            [false, false],
            "least common"
        );
    }

    #[test]
    fn test_iterative_search() {
        let input = as_binary_rows(vec!["00".into(), "00".into(), "00".into(), "11".into()]);
        let result = iterative_search(|_| [true, true], &input);

        assert_eq!(result, Some([true, true]));
    }

    #[test]
    fn test_iterative_search_unordered() {
        let input = as_binary_rows(vec!["00".into(), "00".into(), "11".into(), "00".into()]);
        let result = iterative_search(|_| [true, true], &input);

        assert_eq!(result, Some([true, true]));
    }

    #[test]
    fn test_iterative_search_unordered_partial() {
        let input = as_binary_rows(vec!["00".into(), "00".into(), "10".into(), "00".into()]);
        let result = iterative_search(|_| [true, true], &input);

        assert_eq!(result, Some([true, false]));
    }

    #[test]
    fn test_iterative_search_partial_match() {
        let input = as_binary_rows(vec!["000".into(), "000".into(), "000".into(), "100".into()]);
        let result = iterative_search(|_| [true, true, true], &input);

        assert_eq!(result, Some([true, false, false]));
    }

    #[test]
    fn should_calculate_oxygen_rating_properly() {
        let input: Vec<[bool; 5]> = as_binary_rows([
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ]);

        assert_eq!(oxygen_generator_rating(&input), Some(23));
    }

    #[test]
    fn should_calculate_co2_scrubber_rating_properly() {
        let input: Vec<[bool; 5]> = as_binary_rows([
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ]);

        assert_eq!(co2_scrubber_rating(&input), Some(10));
    }
}
