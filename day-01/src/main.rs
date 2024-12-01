#[allow(unused_imports)]
use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    let regex = Regex::new(r"(\d+)\s+(\d+)\n").unwrap();
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for c in regex.captures_iter(raw_input) {
        list1.push(c[1].parse::<i64>().unwrap());
        list2.push(c[2].parse::<i64>().unwrap());
    }
    let mut sort1= list1.clone();
    let mut sort2= list2.clone();
    sort1.sort_unstable();
    sort2.sort_unstable();

    let diff: i64 = sort1.iter().zip(sort2.iter()).map(|(a,b)| (a-b).abs()).sum();
    println!("part1: {diff}");

    let mut sum = 0usize;
    for d in list1.iter() {
        sum += *d as usize * list2.iter().filter(|d2| *d2 == d).count();
    }
    println!("part2: {sum}");
    Ok(())
}
