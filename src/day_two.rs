use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct MovementParseError;

impl Display for MovementParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid movement")
    }
}

impl Error for MovementParseError {}

pub struct Movement(Direction, i32);

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ')
            .and_then(|(dir, dist)| {
                let dir = dir.parse().ok();
                let dist = dist.parse().ok();

                dir.zip(dist).map(|(a, b)| Movement(a, b))
            })
            .ok_or(MovementParseError)
    }
}

pub fn process_instructions(
    starting_depth: i32,
    starting_position: i32,
    instructions: &[Movement],
) -> (i32, i32) {
    instructions.iter().fold(
        (starting_depth, starting_position),
        |(depth, position), Movement(dir, dist)| match dir {
            Direction::Forward => (depth, position + dist),
            Direction::Down => (depth + dist, position),
            Direction::Up => (depth - dist, position),
        },
    )
}
