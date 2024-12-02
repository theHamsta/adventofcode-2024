//#![feature(array_windows)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    //let regex = Regex::new(r"(\d+)\s+(\d+)\n").unwrap();
    let mut grid = Vec::new();

    for line in raw_input.lines().filter(|l| !l.is_empty()) {
        grid.push(
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec(),
        );
    }

    let mut safe = 0i64;
    let mut safe2 = 0i64;
    for line in grid.iter() {
        let is_safe2 = (0..line.len()).find(|skip| {
            let is_inc = line
                .iter()
                .enumerate()
                .filter_map(|(i, &value)| (i != *skip).then_some(value))
                .tuple_windows()
                .map(|(a, b)| b - a)
                .all(|diff| (1..=3).contains(&diff));
            let is_dec = line
                .iter()
                .enumerate()
                .filter_map(|(i, &value)| (i != *skip).then_some(value))
                .tuple_windows()
                .map(|(a, b)| b - a)
                .all(|diff| (-3..=-1).contains(&diff));
            is_dec || is_inc
        });

        let is_inc = line
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .all(|diff| (1..=3).contains(&diff));
        let is_dec = line
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .all(|diff| (-3..=-1).contains(&diff));
        if is_dec || is_inc {
            safe += 1;
        }
        if is_safe2.is_some() || is_dec || is_inc {
            safe2 += 1;
        }
    }
    println!("{safe}");
    println!("{safe2}");

    Ok(())
}
