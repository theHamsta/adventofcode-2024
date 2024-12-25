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


fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example");
    let grids = raw_input
        .split("\r\n\r\n")
        .map(|g| {
            g.lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().collect_vec())
                .collect_vec()
        })
    .filter(|g| g.len() > 1)
        .collect_vec();
    dbg!(&grids.len());

    let mut sum:i64 = 0;
    for (a,b) in grids.iter().tuple_combinations() {
        let mut overlap = false;
        for y in 0..a.len() {
            for x in 0..a[0].len() {
                if a[y][x] == '#' && b[y][x] == '#' {
                    overlap = true;
                }

            }
        }
        if !overlap {
            sum +=1;
        }
    }
    dbg!(&sum);


    Ok(())
}
