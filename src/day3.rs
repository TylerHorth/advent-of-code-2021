use std::{collections::HashSet, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};

const NUM_BITS: usize = 12;

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Vec<u16>, ParseIntError> {
    input
        .lines()
        .map(|l| u16::from_str_radix(l, 2))
        .collect()
}

fn most_common<'a, T, I>(input: I, mask: u16) -> bool
    where   
        I: IntoIterator<Item = &'a u16, IntoIter = T>,
        T: Iterator<Item = &'a u16>,
        T: ExactSizeIterator {
    let iter = input.into_iter();
    let len = iter.len();
    let ones = iter.filter(|&&n| n & mask > 0).count();

    ones + ones >= len
}

#[aoc(day3, part1)]
fn part1(input: &[u16]) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    for bit in (0..NUM_BITS).rev() {
        gamma <<= 1;
        epsilon <<= 1;

        if most_common(input, 1 << bit) {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn find_mask(input: &[u16], invert: bool) -> u16 {
    let mut set: HashSet<u16> = input.iter().copied().collect();
    for bit in (0..NUM_BITS).rev() {
        let mask = 1u16 << bit;
        let most_common = most_common(&set, mask) ^ invert;

        set.retain(|&n| (n & mask > 0) == most_common);

        if set.len() == 1 {
            return *set.iter().next().unwrap();
        }
    }

    panic!("no solution")
}

#[aoc(day3, part2)]
fn part2(input: &[u16]) -> u32 {
    let ox = find_mask(input, false) as u32;
    let co2 = find_mask(input, true) as u32;

    ox * co2
}