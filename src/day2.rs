use parse_display::{Display, FromStr, ParseError};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Display, FromStr)]
#[display("{} {0}", style = "lowercase")]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Command>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Command>) -> u32 {
    let mut depth = 0;
    let mut position = 0;

    for m in input {
        match m {
            Command::Forward(val) => position += val,
            Command::Down(val) => depth += val,
            Command::Up(val) => depth -= val,
        }
    }

    position * depth
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Command>) -> u32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut position = 0;

    for m in input {
        match m {
            Command::Forward(val) => { 
                position += val;
                depth += aim * val;
            },
            Command::Down(val) => aim += val,
            Command::Up(val) => aim -= val,
        }
    }

    position * depth
}