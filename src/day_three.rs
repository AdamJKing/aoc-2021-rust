pub fn gamma_and_epsilon_rates<const S: usize>(
    bits: impl IntoIterator<Item = [bool; S]>,
) -> (u32, u32) {
    let mut total = 0;
    let mut counts = [0; S];

    for row in bits {
        for i in 0..S {
            counts[i] += row[i] as i32;
        }
        total += 1;
    }

    let gamma_rate = build_binary(counts.iter().map(|&c| c >= (total - c)));
    let epsilon_rate = build_binary(counts.iter().map(|&c| c <= (total - c)));

    (gamma_rate, epsilon_rate)
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

fn build_binary(xs: impl DoubleEndedIterator<Item = bool>) -> u32 {
    let mut bin = 0;
    for (i, is_bit) in xs.rev().enumerate() {
        if is_bit {
            bin += u32::pow(2, i as u32);
        }
    }

    bin
}

#[cfg(test)]
mod tests {
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

        let (gamma, _) = gamma_and_epsilon_rates(input);
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

        let (_, epsilon) = gamma_and_epsilon_rates(input);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn should_build_binary() {
        let result = build_binary([true, false, true, true, false, true].into_iter());
        assert_eq!(result, 0b101101);
    }
}
