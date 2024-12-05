//#![feature(array_windows)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    let rules_regex = Regex::new(r"([-]?\d+)\|([-]?\d+)").unwrap();
    let numbers = Regex::new(r"([-]?\d+),.*").unwrap();

    let mut rules = HashMap::<i64, Vec<i64>>::new();
    for r in rules_regex
        .captures_iter(raw_input)
        .map(|c| (c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()))
    {
        rules.entry(r.0).or_default().push(r.1);
    }
    let numbers = numbers
        .captures_iter(raw_input)
        .map(|c| {
            let line = &c[0];
            line.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let part1: i64 = numbers
        .iter()
        .filter(|numbers| {
            for (i, &n) in numbers.iter().enumerate() {
                let requirements = rules.get(&n);
                let before = &numbers[..i];
                if let Some(requirements) = requirements {
                    for requirement in requirements.iter() {
                        if before.iter().any(|&f| f == *requirement) {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .map(|numbers| {
            let middle = numbers.len() / 2;
            numbers[middle]
        })
        .sum();
    dbg!(&part1);

    let part2: i64 = numbers
        .iter()
        .filter(|numbers| {
            for (i, &n) in numbers.iter().enumerate() {
                let requirements = rules.get(&n);
                let before = &numbers[..i];
                if let Some(requirements) = requirements {
                    for requirement in requirements.iter() {
                        if before.iter().any(|&f| f == *requirement) {
                            return true;
                        }
                    }
                }
            }
            false
        })
        .map(|numbers| {
            let mut numbers = numbers.clone();
            numbers.sort_unstable_by(|a, b| {
                let requirements = rules.get(a);
                if let Some(requirements) = requirements {
                    let found = requirements.iter().find(|number| *number == b);
                    if found.is_some() {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            });
            numbers
        })
        .map(|numbers| {
            let middle = numbers.len() / 2;
            numbers[middle]
        })
        .sum();
    dbg!(&part2);

    //let grains = regex
    //.captures_iter(input)
    //.map(|c| HailGrain {

    Ok(())
}
