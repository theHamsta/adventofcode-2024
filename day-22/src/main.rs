#![feature(map_try_insert)]
#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use regex::Regex;
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

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    secret: i64,
    prize: i64,
    delta: i64,
}

fn simulate(mut secret: i64, steps: i64) -> i64 {
    for _ in 0..steps {
        let mut next = secret * 64;
        secret ^= next;
        secret %= 16777216;

        next = secret / 32;
        secret ^= next;
        secret %= 16777216;

        next = secret * 2048;
        secret ^= next;
        secret %= 16777216;
    }

    secret
}

fn simulate_all(mut secret: i64, steps: i64) -> Vec<State> {
    let mut rtn = Vec::new();

    for _ in 0..steps {
        let old_secret = secret;
        let mut next = secret * 64;
        secret ^= next;
        secret %= 16777216;

        next = secret / 32;
        secret ^= next;
        secret %= 16777216;

        next = secret * 2048;
        secret ^= next;
        secret %= 16777216;
        rtn.push(State {
            secret,
            prize: secret % 10,
            delta: (old_secret % 10) - (secret % 10),
        })
    }

    rtn
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example2");

    let input = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect_vec();

    let part1: i64 = input.iter().map(|&i| simulate(i, 2000)).sum();
    dbg!(&part1);

    let part2_seq = input.iter().map(|&i| simulate_all(i, 2000)).collect_vec();

    let mut diff_storage = HashMap::new();

    for (i, states) in part2_seq.iter().enumerate() {
        for (a, b, c, d) in states.iter().tuple_windows() {
            let _ = diff_storage
                .entry((a.delta, b.delta, c.delta, d.delta))
                .or_insert_with(HashMap::new)
                .try_insert(i, d.prize);
        }
    }
    let part2 = diff_storage
        .iter()
        .map(|(k, v)| (k, v.values().sum::<i64>()))
        //.map(|(k, v)| (k, (v, v.values().sum::<i64>())))
        .max_by_key(|(_k, v)| *v);
    dbg!(&part2);

    //let mut state = State{secret:123, next:0};
    //for _ in 0..10 {
    //state = simulate(state.secret);
    //dbg!(&state);
    //}

    //1450
    Ok(())
}
