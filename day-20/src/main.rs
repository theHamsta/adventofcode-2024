#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
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
fn print_grid(grid: &[Vec<char>]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    println!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Normal,
    WallSearch,
    WallEnter,
}

fn bfs(
    start: (i16, i16),
    grid: &[Vec<char>],
    mode: Mode,
    max_cheat: i16,
) -> HashMap<(i16, i16), i16> {
    let mut visited = HashMap::new();

    let mut todo = BinaryHeap::new();
    todo.push(Reverse((0i16, start.0, start.1)));
    visited.insert(start, 0);

    while let Some(Reverse((cost, x, y))) = todo.pop() {
        for (dx, dy) in [(1i16, 0i16), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || nx >= grid[0].len() as i16 || ny < 0 || ny >= grid.len() as i16 {
                continue;
            }
            let neighbor = grid[ny as usize][nx as usize];
            let allowed = match mode {
                Mode::Normal => neighbor == '.',
                Mode::WallSearch => true,
                Mode::WallEnter => true,
            };
            if !allowed {
                continue;
            }

            match visited.entry((nx, ny)) {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    assert!(*occupied_entry.get() <= cost + 1);
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    if mode == Mode::WallSearch && cost + 1 > max_cheat {
                        continue;
                    }
                    vacant_entry.insert(cost + 1);
                    if mode == Mode::WallEnter && neighbor == '#' {
                        continue;
                    }
                    todo.push(Reverse((cost + 1, nx, ny)));
                }
            }
        }
    }

    visited
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");

    //let raw_input = include_str!("../example");

    let mut grid = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();
    print_grid(&grid);

    let mut start_pos = (0i16, 0i16);
    let mut end_pos = (0i16, 0i16);
    let mut wall_pos = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start_pos = (x as i16, y as i16);
                grid[y][x] = '.';
            } else if grid[y][x] == 'E' {
                end_pos = (x as i16, y as i16);
                grid[y][x] = '.';
            } else if grid[y][x] == '#' {
                wall_pos.push((x as i16, y as i16));
            }
        }
    }

    for max_cheat in [2, 20] {
        let non_cheated = bfs(start_pos, &grid, Mode::Normal, max_cheat);
        let normal_cost = non_cheated[&end_pos];
        //dbg!(&normal_cost);

        let end_to_cheat_exit = bfs(end_pos, &grid, Mode::Normal, max_cheat);
        //dbg!(&end_to_cheat_exit.len());

        let cheat_delta = 100;
        let mut count = 0;
        let mut dict: HashMap<i16, i16> = HashMap::new();

        for (enter_pos, &enter_cost) in non_cheated.iter() {
            let result = bfs(*enter_pos, &grid, Mode::WallSearch, max_cheat);
            for (exit, &cost) in result.iter() {
                if let Some(end_to_exit) = end_to_cheat_exit.get(exit) {
                    let total = enter_cost + cost + end_to_exit;
                    if total <= normal_cost - cheat_delta {
                        count += 1;
                        *dict.entry(normal_cost - total).or_default() += 1;
                    }
                }
            }
        }
        dbg!(&count);
    }

    Ok(())
}
