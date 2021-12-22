mod day_one;

use day_one::count_increases;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day One");
    {
        let input = iterate_lines::<i32>("data/day_one.txt")?;
        println!("Part One {}", count_increases(input));
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
