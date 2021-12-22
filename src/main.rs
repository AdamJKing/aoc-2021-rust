mod day_one;
mod day_two;

use day_one::count_increases;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::day_two::{process_instructions, Movement, Submarine};

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
        println!("Part One depth: {} position: {}", depth, position);
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
