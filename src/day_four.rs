use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

struct Board {
    numbers: [[u32; 5]; 5],
    wins: [[bool; 5]; 5],
}

struct Bingo {
    numbers: Vec<u32>,
}

struct FileParseError(&'static str);

fn load_file(filename: &str) -> Result<Bingo, FileParseError> {
    let file = File::open(filename).expect("Mising input file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let calls = lines
        .next()
        .ok_or(FileParseError("(Day 4) Input did not contain bingo calls"))?;

    while lines.next().is_some() {
        lines.n
        read_board(lines)
    }
    
}

fn read_board<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Board, FileParseError> {
    fn parse_row(row: &str) -> Result<[u32; 5], FileParseError> {
        row.split_whitespace()
            .take(5)
            .map(|column| {
                column
                    .parse::<u32>()
                    .map_err(|_| FileParseError("Found a non u32 in the input file"))
            })
            .collect::<Result<Vec<u32>, FileParseError>>()
            .and_then(|r| {
                r.try_into()
                    .map_err(|_| FileParseError("too many/not enough columns"))
            })
    }

    lines
        .take(5)
        .map(|row| parse_row(row))
        .collect::<Result<Vec<[u32; 5]>, FileParseError>>()
        .and_then(|r| {
            r.try_into()
                .map_err(|_| FileParseError("too many/not enough columns"))
        })
        .map(|numbers| Board {
            numbers,
            wins: [[false; 5]; 5],
        })
}
