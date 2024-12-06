//#![feature(array_windows)]
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

fn turn_right((dx, dy): (i64, i64)) -> (i64, i64) {
    let (dy, dx) = match (dy, dx) {
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        _ => unreachable!(),
    };
    (dx, dy)
}

#[derive(Debug)]
enum Outcome {
    Leave(usize),
    Loop,
}

impl Outcome {
    /// Returns `true` if the outcome is [`Loop`].
    ///
    /// [`Loop`]: Outcome::Loop
    #[must_use]
    fn is_loop(&self) -> bool {
        matches!(self, Self::Loop)
    }
}

fn do_protocol(
    start_pos: (i64, i64),
    grid: &Vec<Vec<char>>,
    obstacle: Option<(usize, usize)>,
) -> Outcome {
    let mut grid = grid.to_owned();
    let mut dir = (0i64, -1i64);
    let mut pos = start_pos;
    let mut visited = 0usize;
    let mut state = HashMap::new();
    if let Some(obstacle) = obstacle {
        grid[obstacle.1][obstacle.0] = '#';
    }
    loop {
        let (x, y) = pos;
        let (dx, dy) = dir;
        if grid[y as usize][x as usize] != 'X' {
            grid[y as usize][x as usize] = 'X';
            visited += 1;
        }
        match state.entry((pos, dir)) {
            std::collections::hash_map::Entry::Occupied(_) => return Outcome::Loop,
            std::collections::hash_map::Entry::Vacant(vacant_entry) => vacant_entry.insert(1),
        };

        let mut next = (x + dx, y + dy);
        if next.0 < 0 || next.0 >= grid[0].len() as i64 || next.1 < 0 || next.1 >= grid.len() as i64
        {
            return Outcome::Leave(visited);
        }
        let mut next_field = grid[next.1 as usize][next.0 as usize];

        while next_field == '#' {
            let (dx, dy) = turn_right(dir);
            dir = (dx, dy);
            next = (x + dx, y + dy);
            next_field = grid[next.1 as usize][next.0 as usize];
            //if next_field == '#' {
            //print_grid(&grid);
            //}
        }
        //print_grid(&grid);

        pos = next;
    }
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    let grid = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    print_grid(&grid);

    let mut start_pos = (0i64, 0i64);
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                start_pos = (x as i64, y as i64);
            }
        }
    }

    let visited = do_protocol(start_pos, &grid, None);
    dbg!(&visited);

    let mut sum = 0usize;
    for y in 0..grid.len() {
        println!("{y}/{}", grid.len());
        for x in 0..grid[0].len() {
            if do_protocol(start_pos, &grid, Some((x, y))).is_loop() {
                sum += 1;
            }
        }
    }
    dbg!(&sum);

    Ok(())
}
