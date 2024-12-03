//#![feature(array_windows)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    let regex = Regex::new(r"mul[(]([-]?\d+),([-]?\d+)[)]").unwrap();

    let sum: i64 = regex
        .captures_iter(raw_input)
        .map(|c| {
            let n1 = c[1].parse::<i64>().unwrap();
            let n2 = c[2].parse::<i64>().unwrap();
            n1 * n2
        })
        .sum();
    println!("{sum}");

    let regex = Regex::new(r"(mul[(]([-]?\d+),([-]?\d+)[)]|do\(\)|don't\(\))").unwrap();

    let mut sum = 0i64;
    let mut enable: bool = true;
    for instruction in regex.captures_iter(raw_input) {
        let whole = &instruction[0];
        if whole.starts_with("mul") && enable {
            let n1 = instruction[2].parse::<i64>().unwrap();
            let n2 = instruction[3].parse::<i64>().unwrap();
            sum += n1 * n2;
        } else if whole.starts_with("don't") {
            enable = false;
        } else if whole.starts_with("do") {
            enable = true;
        }
    }
    
    println!("{sum}");
    Ok(())
}
