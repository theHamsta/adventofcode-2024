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

    let grid = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    //print_grid(&grid);

    let mut visited = HashSet::new();
    let mut areas = HashMap::<(i64, i64), HashSet<(i64, i64)>>::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            } else {
                visited.insert((x, y));
            }

            let cur = grid[y][x];
            let region = areas.entry((x as i64, y as i64)).or_default();
            region.insert((x as i64, y as i64));

            let mut todo = vec![(x as i64, y as i64)];
            while let Some(pixel) = todo.pop() {
                for (dx, dy) in [(1i64, 0i64), (0, 1), (-1, 0), (0, -1)] {
                    let (x, y) = (pixel.0 + dx, pixel.1 + dy);
                    if x >= 0
                        && x < grid[0].len() as i64
                        && y >= 0
                        && y < grid.len() as i64
                        && grid[y as usize][x as usize] == cur
                        && !visited.contains(&(x as usize, y as usize))
                    {
                        todo.push((x, y));
                        region.insert((x, y));
                        visited.insert((x as usize, y as usize));
                    }
                }
            }
        }
    }

    let part1: i64 = areas
        .iter()
        .map(|(k, v)| {
            let area = v.len();
            let mut peri = 0i64;
            for pixel in v.iter() {
                for (dx, dy) in [(1i64, 0i64), (0, 1), (-1, 0), (0, -1)] {
                    let (x, y) = (pixel.0, pixel.1);

                    let neighbor = (x + dx, y + dy);
                    if !v.contains(&neighbor) {
                        peri += 1;
                    }
                }
            }
            area as i64 * peri
        })
        .sum();
    dbg!(&part1);

    let part2: i64 = areas
        .iter()
        .map(|(k, v)| {
            let area = v.len();
            let mut peri_pixels = HashSet::new();
            for pixel in v.iter() {
                for (dx, dy) in [(1i64, 0i64), (0, 1), (-1, 0), (0, -1)] {
                    let (x, y) = (pixel.0, pixel.1);

                    let neighbor = (x + dx, y + dy);
                    if !v.contains(&neighbor) {
                        peri_pixels.insert((x, y, dx, dy));
                    }
                }
            }
            let mut sides = 0i64;
            while let Some(&(x, y, dx, dy)) = peri_pixels.iter().next() {
                sides += 1;
                for i in 0i64.. {
                    if dx == 0 {
                        let contained = peri_pixels.remove(&(x + i, y, dx, dy));
                        if !contained {
                            break;
                        }
                    }
                    if dy == 0 {
                        let contained = peri_pixels.remove(&(x, y + i, dx, dy));
                        if !contained {
                            break;
                        }
                    }
                }
                for i in 1i64.. {
                    if dx == 0 {
                        let contained = peri_pixels.remove(&(x - i, y, dx, dy));
                        if !contained {
                            break;
                        }
                    }
                    if dy == 0 {
                        let contained = peri_pixels.remove(&(x, y - i, dx, dy));
                        if !contained {
                            break;
                        }
                    }
                }
            }
            area as i64 * sides
        })
        .sum();
    dbg!(&part2);

    Ok(())
}
