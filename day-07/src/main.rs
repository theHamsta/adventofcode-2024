use itertools::repeat_n;
//#![feature(array_windows)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug)]
struct Caclculation {
    result: i64,
    numbers: Vec<i64>,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Cat,
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    let input = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut it = l.split_whitespace();
            let result = it.next().unwrap();
            let result = result.split(':').next().unwrap();
            let result = result.parse::<i64>().unwrap();
            let numbers = it.map(|n| n.parse().unwrap()).collect_vec();
            Caclculation { result, numbers }
        })
        .collect_vec();

    let part1: i64 = input
        .iter()
        .filter_map(|c| {
            let mut perms =
                repeat_n([Op::Add, Op::Mul].iter(), c.numbers.len() - 1).multi_cartesian_product();

            perms
                .any(|p| {
                    let mut res = c.numbers[0];
                    for (op, n) in p.iter().zip(c.numbers[1..].iter()) {
                        match op {
                            Op::Add => res += n,
                            Op::Mul => res *= n,
                            _ => unreachable!(),
                        }
                    }
                    res == c.result
                })
                .then_some(c.result)
        })
        .sum();
    dbg!(part1);

    let part2: i64 = input
        .iter()
        .filter_map(|c| {
            let mut perms = repeat_n([Op::Add, Op::Mul, Op::Cat].iter(), c.numbers.len() - 1)
                .multi_cartesian_product();

            perms
                .any(|p| {
                    let mut res = c.numbers[0];
                    for (op, n) in p.iter().zip(c.numbers[1..].iter()) {
                        match op {
                            Op::Add => res += n,
                            Op::Mul => res *= n,
                            Op::Cat => res = format!("{res}{n}").parse().unwrap(),
                        }
                    }
                    res == c.result
                })
                .then_some(c.result)
        })
        .sum();
    dbg!(part2);

    Ok(())
}
