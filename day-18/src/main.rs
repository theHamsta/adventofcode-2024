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
fn print_grid(corrupted: &HashMap<(i64, i64), i64>, path: &HashSet<(i64, i64)>, grid_size: i64) {
    for y in 0..grid_size {
        for x in 0..grid_size {
            if let Some(time) = corrupted.get(&(x, y)) {
                print!("#");
            } else if path.contains(&(x, y)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    let grid_size = 71;

    //let raw_input = include_str!("../example");
    //let grid_size = 7;

    let coords: HashMap<(i64, i64), i64> = raw_input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| {
            (!l.is_empty()).then_some({
                let mut it = l.split(',');
                let x: i64 = it.next().unwrap().parse().unwrap();
                let y: i64 = it.next().unwrap().parse().unwrap();
                ((x, y), i as i64 + 1)
            })
        })
        .collect();

    let start_pos = (0i64, 0i64);
    let end_pos = (grid_size - 1, grid_size - 1);

    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    heap.push(Reverse((0i64, start_pos.0, start_pos.1)));
    visited.insert(start_pos, (0, 0, 0));

    while let Some(Reverse((cost, x, y))) = heap.pop() {
        if (x, y) == end_pos {
            break;
        }
        for (dx, dy) in [(0i64, 1i64), (1, 0), (-1, 0), (0, -1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= grid_size || ny >= grid_size {
                continue;
            }
            match coords.get(&(nx, ny)) {
                //Some(&fall_time) if fall_time > cost + 1 => {}
                Some(&fall_time) if fall_time <= 1024 => {
                    continue;
                }
                Some(_) => {}
                None => {}
            }
            match visited.entry((nx, ny)) {
                std::collections::hash_map::Entry::Occupied(_o) => {
                    continue;
                }
                std::collections::hash_map::Entry::Vacant(v) => v.insert((cost + 1, x, y)),
            };

            heap.push(Reverse((cost + 1, nx, ny)));
        }
    }

    let part1 = visited[&end_pos];
    dbg!(&part1);

    let first_blocked_step = (0..coords.len()).find(|i| {
        let mut heap = BinaryHeap::new();
        let mut visited = HashMap::new();
        heap.push(Reverse((0i64, start_pos.0, start_pos.1)));
        visited.insert(start_pos, (0, 0, 0));

        while let Some(Reverse((cost, x, y))) = heap.pop() {
            if (x, y) == end_pos {
                break;
            }
            for (dx, dy) in [(0i64, 1i64), (1, 0), (-1, 0), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= grid_size || ny >= grid_size {
                    continue;
                }
                match coords.get(&(nx, ny)) {
                    //Some(&fall_time) if fall_time > cost + 1 => {}
                    Some(&fall_time) if fall_time <= *i as i64 => {
                        continue;
                    }
                    Some(_) => {}
                    None => {}
                }
                match visited.entry((nx, ny)) {
                    std::collections::hash_map::Entry::Occupied(_o) => {
                        continue;
                    }
                    std::collections::hash_map::Entry::Vacant(v) => v.insert((cost + 1, x, y)),
                };

                heap.push(Reverse((cost + 1, nx, ny)));
            }
        }
        !visited.contains_key(&end_pos)
    });
    let part2 = coords
        .iter()
        .find(|(_, v)| **v == first_blocked_step.unwrap() as i64)
        .map(|(k, _)| k);
    dbg!(&part2);

    Ok(())
}
