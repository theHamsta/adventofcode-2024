//#![feature(array_windows)]
#![feature(new_range_api)]
use core::range::Range;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;

fn print_grid(grid: &[Vec<char>]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    println!();
}

fn get_height(grid: &[Vec<char>], (x, y): (i64, i64)) -> i64 {
    grid[y as usize][x as usize] as i64 - '0' as i64
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example");

    let grid = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    print_grid(&grid);

    let mut start_positions = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '0' {
                start_positions.push((x as i64, y as i64));
            }
        }
    }

    let mut score = 0i64;
    let mut rating = 0i64;
    for start in start_positions.iter() {
        let mut visited = HashMap::new();

        let mut todo = vec![*start];
        let mut partial_score = 0i64;
        let mut partial_rating = 0i64;

        while let Some(cur) = todo.pop() {
            let (x, y) = cur;
            match visited.entry(cur) {
                std::collections::hash_map::Entry::Occupied(mut occupied) => {
                    if grid[y as usize][x as usize] == '9' {
                        *occupied.get_mut() += 1;
                        partial_rating += 1;
                    }
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    if grid[y as usize][x as usize] == '9' {
                        partial_score += 1;
                        partial_rating += 1;
                    }
                    vacant_entry.insert(1);
                }
            };
            for n in [(1i64, 0i64), (0, 1), (-1, 0), (0, -1)] {
                let next = (cur.0 + n.0, cur.1 + n.1);
                //let mut was_there_already = false;
                if next.0 >= 0
                    && next.0 < grid[0].len() as i64
                    && next.1 >= 0
                    && next.1 < grid.len() as i64
                    && get_height(&grid, next) - get_height(&grid, cur) == 1
                //&& !was_there_already
                {
                    todo.push(next);
                }
            }
        }
        //dbg!(&partial_rating);
        score += partial_score;
        rating += partial_rating;
    }
    dbg!(score);
    dbg!(rating);

    Ok(())
}
