#![feature(hash_set_entry)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    println!();
}

fn check<'string>(
    desired: &'string str,
    avialable: &HashMap<usize, HashSet<&str>>,
    cache: &mut HashMap<&'string str, i64>,
) -> i64 {
    if desired.is_empty() {
        return 1;
    }
    if let Some(lookup) = cache.get(desired) {
        return *lookup;
    }

    let mut sum = 0;
    for (&c, per_count) in avialable {
        if c <= desired.len() {
            let (prefix, remaining) = desired.split_at(c);
            if per_count.contains(prefix) {
                sum += check(remaining, avialable, cache);
            }
        }
    }
    cache.insert(desired, sum);
    sum
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    //let raw_input = include_str!("../example");

    let mut it = raw_input.lines().filter(|l| !l.is_empty());

    let avialable: HashSet<_> = it.next().unwrap().split(", ").collect();
    let desired = it.collect_vec();
    let mut counts: HashMap<usize, HashSet<&str>> = HashMap::new();
    avialable.iter().for_each(|v| {
        counts.entry(v.len()).or_default().insert(v);
    });
    //dbg!(&counts);
    //dbg!(&avialable);
    //dbg!(&desired);

    let mut cache = HashMap::new();
    let part1 = desired
        .iter()
        .filter(|&d| check(d, &counts, &mut cache) > 0)
        .count();
    dbg!(&part1);
    let part2: i64 = desired
        .iter()
        .map(|&d| check(d, &counts, &mut cache))
        .sum();
    dbg!(&part2);

    Ok(())
}
