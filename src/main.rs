mod day_one;
mod day_three;
mod day_two;

use day_one::count_increases;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::{
    day_three::{
        as_binary_rows, co2_scrubber_rating, gamma_and_epsilon_rates, oxygen_generator_rating,
    },
    day_two::{process_instructions, Movement, Submarine},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day One");
    {
        let input = iterate_lines::<i32>("data/day_one.txt")?;
        println!("Part One {}", count_increases(&input, 1));
        println!("Part Two {}", count_increases(&input, 3));
    }

    println!("Day Two");
    {
        let input = iterate_lines::<Movement>("data/day_two.txt")?;
        let Submarine {
            depth, position, ..
        } = process_instructions(Submarine::default(), &input);
        println!("depth: {} position: {}", depth, position);
    }

    println!("Day Three");
    {
        let input = iterate_lines::<String>("data/day_three.txt")?;
        let bits: Vec<[bool; 12]> = as_binary_rows(input);
        let (gamma, epsilon) = gamma_and_epsilon_rates(&bits);
        println!("gamma: {} epsilon: {}", gamma, epsilon);
        let oxy_gen_rating = oxygen_generator_rating(&bits);
        let co2_scrub_rating = co2_scrubber_rating(&bits);
        println!(
            "oxy-gen-rating: {:?}, co2-scrub-rating: {:?}",
            oxy_gen_rating, co2_scrub_rating
        );
    }

    Ok(())
}

fn iterate_lines<T>(filename: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static + Error,
{
    let file = File::open(filename).expect("Mising input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|x| {
            x.map_err(|err| err.into())
                .and_then(|x| x.parse::<T>().map_err(|err| err.into()))
        })
        .collect()
}
