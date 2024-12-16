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

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example2");

    let grid = raw_input
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();
    print_grid(&grid);

    let mut start_tile = (0i64, 0i64);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start_tile = (x as i64, y as i64);
            }
        }
    }
    dbg!(&start_tile);

    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, start_tile.0, start_tile.1, 1i64, 0i64)));

    let mut visited = HashMap::new();

    let mut end_paths = Vec::new();
    let mut end_cost = i64::MAX;
    while let Some(Reverse((cost, x, y, dirx, diry))) = heap.pop() {
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1i64, 0i64)] {
            let (nx, ny) = (x + dx, y + dy);
            let neighbor = grid[ny as usize][nx as usize];
            if neighbor == '#' {
                continue;
            }

            let cost_inc = match (dirx, diry) {
                (0, 0) => 1001,
                (dirx, diry) if dirx == dx && diry == dy => 1,
                (dirx, diry) if dirx == -dx && diry == -dy => 2001,
                _ => 1001,
            };
            let new_cost = cost + cost_inc;
            match visited.entry((nx, ny, dx, dy)) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    let (other_cost, p): &mut (i64, Vec<(i64, i64, i64, i64)>) =
                        occupied_entry.get_mut();
                    if *other_cost == new_cost {
                        p.push((x, y, dirx, diry));
                        heap.push(Reverse((new_cost, nx, ny, dx, dy)));
                    }
                    if *other_cost > new_cost {
                        *other_cost = new_cost;
                        *p = vec![(x, y, dirx, diry)];
                        heap.push(Reverse((new_cost, nx, ny, dx, dy)));
                    }
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert((new_cost, vec![(x, y, dirx, diry)]));
                    heap.push(Reverse((new_cost, nx, ny, dx, dy)));
                }
            }
            if neighbor == 'E' && new_cost <= end_cost {
                end_paths.push((nx, ny, dx, dy));
                end_cost = new_cost;
            }
        }
    }
    dbg!(&end_cost);

    let mut debug_grid = grid.clone();
    let mut seat_set = HashSet::new();

    let mut backtraced_states = HashSet::new();
    for &cur in end_paths.iter() {
        let mut todo = vec![cur];
        while let Some(cur) = todo.pop() {
            debug_grid[cur.1 as usize][cur.0 as usize] = 'O';
            seat_set.insert((cur.0, cur.1));
            if (cur.0, cur.1) == start_tile {
                continue;
            }
            if !backtraced_states.insert(cur) {
                continue;
            }
            let (_cost, prev) = &visited[&cur];

            for p in prev {
                todo.push(*p);
            }
        }
    }
    let part2 = seat_set.len();
    dbg!(&seat_set.len());
    //print_grid(&debug_grid);

    let mut count = 0i64;
    for y in 0..debug_grid.len() {
        for x in 0..debug_grid[0].len() {
            if debug_grid[y][x] == 'O' {
                count += 1;
            }
        }
    }
    dbg!(&count);
    assert_eq!(count as usize, part2);

    Ok(())
}
