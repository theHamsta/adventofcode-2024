//#![feature(array_windows)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
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

fn count_antinodes(
    grid: &[Vec<char>],
    antenas: &HashMap<char, HashSet<(i64, i64)>>,
    allow_repetitions: bool,
) -> i64 {
    let mut marked_antinodes = grid.to_owned();

    for (_k, v) in antenas.iter() {
        for (a, b) in v.iter().tuple_combinations() {
            let (ax, ay) = *a;
            let (bx, by) = *b;

            let a_to_b = ((bx - ax), (by - ay));

            let mut anti1 = *a;
            let mut anti2 = *b;

            if !allow_repetitions {
                anti1 = (anti1.0 - a_to_b.0, anti1.1 - a_to_b.1);
                anti2 = (anti2.0 + a_to_b.0, anti2.1 + a_to_b.1);
            }

            while anti1.0 >= 0
                && anti1.0 < grid[0].len() as i64
                && anti1.1 >= 0
                && anti1.1 < grid.len() as i64
            {
                marked_antinodes[anti1.1 as usize][anti1.0 as usize] = '#';
                if !allow_repetitions {
                    break;
                }
                anti1 = (anti1.0 - a_to_b.0, anti1.1 - a_to_b.1);
            }
            while anti2.0 >= 0
                && anti2.0 < grid[0].len() as i64
                && anti2.1 >= 0
                && anti2.1 < grid.len() as i64
            {
                marked_antinodes[anti2.1 as usize][anti2.0 as usize] = '#';
                if !allow_repetitions {
                    break;
                }
                anti2 = (anti2.0 + a_to_b.0, anti2.1 + a_to_b.1);
            }
        }
    }
    //print_grid(&marked_antinodes);

    marked_antinodes
        .iter()
        .flatten()
        .filter(|c| **c == '#')
        .count() as i64
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example");

    let grid = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    //print_grid(&grid);

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

    let part1 = count_antinodes(&grid, &antenas, false);
    dbg!(part1);

    let part2 = count_antinodes(&grid, &antenas, true);
    dbg!(part2);

    Ok(())
}
