//#![feature(array_windows)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;

fn count_occurences(text: &str) -> usize {
    text.chars()
        .tuple_windows()
        .filter(|(a, b, c, d)| {
            matches!(
                (*a, *b, *c, *d),
                ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X')
            )
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    let grid = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    assert_eq!(grid.len(), grid[0].len());

    let mut transposed = String::new();

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            transposed.push(grid[y][x]);
        }
        transposed.push('\n');
    }

    let mut diag1 = String::new();
    let mut diag2 = String::new();

    for diag in 0..grid[0].len() {
        for x in 0..(grid[0].len() - diag) {
            diag1.push(grid[x][diag + x]);
            if diag != 0 {
                diag2.push(grid[x + diag][x]);
            }
        }
        diag1.push('\n');
        diag2.push('\n');
    }

    let mut diag3 = String::new();
    let mut diag4 = String::new();

    for diag in 0..grid[0].len() {
        for x in 0..(grid[0].len() - diag) {
            diag3.push(grid[x][grid[0].len() - 1 - x - diag]);
            if diag != 0 {
                diag4.push(grid[x + diag][grid[0].len() - 1 - x]);
            }
        }
        diag3.push('\n');
        diag4.push('\n');
    }

    let transposed_count = count_occurences(&transposed);
    let normal_count = count_occurences(&raw_input);
    let diag1_count = count_occurences(&diag1);
    let diag2_count = count_occurences(&diag2);
    let diag3_count = count_occurences(&diag3);
    let diag4_count = count_occurences(&diag4);

    let sum =
        normal_count + transposed_count + diag1_count + diag2_count + diag3_count + diag4_count;
    dbg!(&sum);

    let mut count = 0usize;
    for y in 1..(grid.len() - 1) {
        for x in 1..(grid.len() - 1) {
            let center = grid[y][x];
            let tripple1 = (grid[y - 1][x - 1], center, grid[y + 1][x + 1]);
            let tripple2 = (grid[y - 1][x + 1], center, grid[y + 1][x - 1]);
            if matches!(tripple1, ('M', 'A', 'S') | ('S', 'A', 'M'))
                && matches!(tripple2, ('M', 'A', 'S') | ('S', 'A', 'M'))
            {
                count += 1;
            }
        }
    }
    println!("{count}");

    Ok(())
}
