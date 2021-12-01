use std::num::ParseIntError;
use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<u32>) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<u32>) -> usize {
    input.windows(3)
        .map(|w| w.iter().sum::<u32>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}