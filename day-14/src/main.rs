#[allow(unused_imports)]
use itertools::Itertools;
use itertools::repeat_n;
#[allow(unused_imports)]
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

#[allow(dead_code)]
fn print_grid(robots: &[(i64, i64, i64, i64)], board_size_x: i64, board_size_y: i64) {
    let mut count = HashMap::<(i64, i64), i64>::new();
    robots.iter().for_each(|r| {
        *count.entry((r.0, r.1)).or_default() += 1i64;
    });
    for y in 0..board_size_y {
        for x in 0..board_size_x {
            if let Some(c) = count.get(&(x, y)) {
                print!("{c}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn calc_treeness(robots: &[(i64, i64, i64, i64)], board_size_x: i64, board_size_y: i64) -> i64 {
    let mut count = HashMap::<(i64, i64), i64>::new();
    robots.iter().for_each(|r| {
        *count.entry((r.0, r.1)).or_default() += 1i64;
    });
    let mut last_row = 0i64;
    let mut cur_treeness = 0i64;
    let mut treeness = 0i64;
    for y in 0..board_size_y {
        let mut row_count = 0;
        for x in 0..board_size_x {
            if count.contains_key(&(x, y)) {
                row_count += 1;
            }
        }
        if row_count > last_row {
            last_row = row_count;
            cur_treeness += 1;
        } else {
            cur_treeness = 0;
        }
        treeness = treeness.max(cur_treeness);
    }
    treeness
}

#[allow(dead_code)]
fn calc_treeness2(robots: &[(i64, i64, i64, i64)], board_size_x: i64, board_size_y: i64) -> i64 {
    let mut count = HashMap::<(i64, i64), i64>::new();
    robots.iter().for_each(|r| {
        *count.entry((r.0, r.1)).or_default() += 1i64;
    });
    let mut max_streak = 0i64;
    for y in 0..board_size_y {
        let mut cur_streak = 0;
        for x in 0..board_size_x {
            if count.contains_key(&(x, y)) {
                cur_streak += 1;
            } else {
                cur_streak = 0;
            }
            max_streak = max_streak.max(cur_streak);
        }
    }
    max_streak
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example");

    let regex = Regex::new(r"p\s*=?([-+]?\d+),\s*?([-+]?\d+) v=([-+]?\d+),\s*?([-+]?\d+)").unwrap();

    let mut robots = Vec::new();
    regex.captures_iter(raw_input).for_each(|c| {
        robots.push((
            c[1].parse::<i64>().unwrap(),
            c[2].parse::<i64>().unwrap(),
            c[3].parse::<i64>().unwrap(),
            c[4].parse::<i64>().unwrap(),
        ));
    });

    let steps = 100;
    let board_size_x = 101;
    let board_size_y = 103;
    let mut cur = robots.clone();
    for _i in 0..steps {
        //dbg!(&cur);
        print_grid(&cur, board_size_x, board_size_y);
        for (x, y, dx, dy) in cur.iter_mut() {
            *x = (*x + *dx + 2 * board_size_x) % board_size_x;
            *y = (*y + *dy + 2 * board_size_y) % board_size_y;
        }
    }

    let mut count = HashMap::<(i64, i64), i64>::new();
    cur.iter().for_each(|r| {
        *count.entry((r.0, r.1)).or_default() += 1i64;
    });

    let mut sum = [0i64; 4];

    for (&(x, y), &c) in count.iter() {
        if x < board_size_x / 2 && y < board_size_y / 2 {
            sum[0] += c;
        }
        if x < board_size_x / 2 && y > board_size_y / 2 {
            sum[1] += c;
        }
        if x > board_size_x / 2 && y < board_size_y / 2 {
            sum[2] += c;
        }
        if x > board_size_x / 2 && y > board_size_y / 2 {
            sum[3] += c;
        }
    }
    let part1: i64 = sum.iter().product();
    dbg!(&part1);

    let mut cur = robots.clone();

    let mut treeness_record = 0i64;
    for steps in 1.. {
        for (x, y, dx, dy) in cur.iter_mut() {
            *x = (*x + *dx + 2 * board_size_x) % board_size_x;
            *y = (*y + *dy + 2 * board_size_y) % board_size_y;
        }

        // treeness2 is true treeness
        let treeness = calc_treeness2(&cur, board_size_x, board_size_y);
        if treeness > treeness_record {
            print_grid(&cur, board_size_x, board_size_y);
            dbg!(steps);
            dbg!(treeness);
            treeness_record = treeness;
        }
    }

    Ok(())
}
