//#![feature(array_windows)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::collections::HashSet;

fn print_grid(grid: &[Vec<char>]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    println!();
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

    let mut antenas: HashMap<char, HashSet<_>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let cur = grid[y][x];
            match cur {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                    antenas.entry(cur).or_default().insert((x as i64, y as i64));
                }
                _ => (),
            };
        }
    }

    let mut marked_antinodes = grid.clone();

    for (_k, v) in antenas.iter() {
        for (a, b) in v.iter().tuple_combinations() {
            let (ax, ay) = *a;
            let (bx, by) = *b;

            let a_to_b = ((bx - ax), (by - ay));

            let anti1 = (a.0 - a_to_b.0, a.1 - a_to_b.1);
            let anti2 = (b.0 + a_to_b.0, b.1 + a_to_b.1);

            if anti1.0 >= 0
                && anti1.0 < grid[0].len() as i64
                && anti1.1 >= 0
                && anti1.1 < grid.len() as i64
            {
                marked_antinodes[anti1.1 as usize][anti1.0 as usize] = '#';
            }
            if anti2.0 >= 0
                && anti2.0 < grid[0].len() as i64
                && anti2.1 >= 0
                && anti2.1 < grid.len() as i64
            {
                marked_antinodes[anti2.1 as usize][anti2.0 as usize] = '#';
            }
        }
    }
    print_grid(&marked_antinodes);

    let mut part1 = 0i64;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if marked_antinodes[y][x] == '#' {
                part1 += 1;
            }
        }
    }

    dbg!(part1);

    let mut marked_antinodes = grid.clone();

    for (_k, v) in antenas.iter() {
        for (a, b) in v.iter().tuple_combinations() {
            let (ax, ay) = *a;
            let (bx, by) = *b;

            let a_to_b = ((bx - ax), (by - ay));

            let mut anti1 = *a;
            let mut anti2 = *b;

            while anti1.0 >= 0
                && anti1.0 < grid[0].len() as i64
                && anti1.1 >= 0
                && anti1.1 < grid.len() as i64
            {
                marked_antinodes[anti1.1 as usize][anti1.0 as usize] = '#';
                anti1 = (anti1.0 - a_to_b.0, anti1.1 - a_to_b.1);
            }
            while anti2.0 >= 0
                && anti2.0 < grid[0].len() as i64
                && anti2.1 >= 0
                && anti2.1 < grid.len() as i64
            {
                marked_antinodes[anti2.1 as usize][anti2.0 as usize] = '#';
                anti2 = (anti2.0 + a_to_b.0, anti2.1 + a_to_b.1);
            }
        }
    }
    print_grid(&marked_antinodes);

    let mut part2 = 0i64;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if marked_antinodes[y][x] == '#' {
                part2 += 1;
            }
        }
    }

    dbg!(part2);

    Ok(())
}