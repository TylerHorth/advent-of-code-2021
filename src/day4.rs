use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> (Vec<u8>, Vec<Vec<Vec<u8>>>) {
    let mut blocks = input.split("\n\n");

    let first: Vec<_> = blocks.next().unwrap().split(",").map(|n| n.parse::<u8>().unwrap()).collect();

    let boards = blocks.map(|b| 
        b.lines().map(|l|
            l.split_whitespace().map(|n| n.parse::<u8>().unwrap()).collect()
        ).collect()
    ).collect();

    (first, boards)
}

fn table(boards: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<bool>>> {
    let mut result = Vec::new();

    for b in boards {
        let mut b_copy = Vec::new();

        for r in b {
            let mut r_copy = Vec::new();

            for _ in r {
                r_copy.push(false);
            }

            b_copy.push(r_copy);
        }

        result.push(b_copy);
    }

    result
}

fn score(board: &Vec<Vec<u8>>, table: &Vec<Vec<bool>>) -> u32 {
    let mut sum = 0;

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if !table[i][j] {
                sum += board[i][j] as u32;
            }
        }
    }

    sum
}

#[aoc(day4, part1)]
fn part1((random_data, boards): &(Vec<u8>, Vec<Vec<Vec<u8>>>)) -> u32 {
    let mut table = table(boards);

    for &i in random_data {
        for b in 0..boards.len() {
            for r in 0..boards[b].len() {
                for c in 0..boards[b][r].len() {
                    if boards[b][r][c] == i {
                        table[b][r][c] = true;

                        let mut complete = true;
                        for x in 0..boards[b].len() {
                            complete &= table[b][x][c];
                        }

                        if complete {
                            return score(&boards[b], &table[b]) * i as u32;
                        }

                        complete = true;
                        for x in 0..boards[b][r].len() {
                            complete &= table[b][r][x];
                        }

                        if complete {
                            return score(&boards[b], &table[b]) * i as u32;
                        }
                    }
                }
            }
        }
    }

    panic!("no solution")
}

#[aoc(day4, part2)]
fn part2((random_data, boards): &(Vec<u8>, Vec<Vec<Vec<u8>>>)) -> u32 {
    let mut table = table(boards);
    let mut won = HashSet::new();

    for &i in random_data {
        for b in 0..boards.len() {
            for r in 0..boards[b].len() {
                for c in 0..boards[b][r].len() {
                    if boards[b][r][c] == i {
                        table[b][r][c] = true;

                        let mut complete = true;
                        for x in 0..boards[b].len() {
                            complete &= table[b][x][c];
                        }

                        if complete {
                            won.insert(b);
                            if won.len() == boards.len() {
                                return score(&boards[b], &table[b]) * i as u32;
                            } 
                        }

                        complete = true;
                        for x in 0..boards[b][r].len() {
                            complete &= table[b][r][x];
                        }

                        if complete {
                            won.insert(b);
                            if won.len() == boards.len()  {
                                return score(&boards[b], &table[b]) * i as u32;
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("no solution")
}