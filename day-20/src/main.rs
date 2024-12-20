#![feature(hash_set_entry)]
#[allow(unused_imports)]
use itertools::Itertools;
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
fn print_grid(grid: &[Vec<char>], state: (i16, i16, CheatState)) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x as i16 == state.0 && y as i16 == state.1 {
                let c = match state.2 {
                    CheatState::Cheatless => 'O',
                    CheatState::CheatEntered { .. } => 'E',
                    CheatState::CheatEnteredInfinite { .. } => 'I',
                    CheatState::Cheated { .. } => 'P',
                };
                print!("{c}");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum CheatState {
    Cheatless,
    CheatEntered { enter: (i16, i16), steps: i16 },
    CheatEnteredInfinite { enter: (i16, i16) },
    Cheated { enter: (i16, i16), exit: (i16, i16) },
}

impl CheatState {
    /// Returns `true` if the cheat state is [`Cheatless`].
    ///
    /// [`Cheatless`]: CheatState::Cheatless
    #[must_use]
    fn is_cheatless(&self) -> bool {
        matches!(self, Self::Cheatless)
    }

    /// Returns `true` if the cheat state is [`Cheated`].
    ///
    /// [`Cheated`]: CheatState::Cheated
    #[must_use]
    fn is_cheated(&self) -> bool {
        matches!(self, Self::Cheated { .. })
    }
}

fn do_search(
    grid: &[Vec<char>],
    start: (i16, i16),
    max_cost: Option<i16>,
    forbidden_cheats: &HashSet<CheatState>,
    cheat_entry: Option<(i16, i16)>,
    cheat_exit: Option<(i16, i16)>,
    max_cheat_steps: i16,
) -> HashMap<(i16, i16), (i16, Vec<(i16, i16, CheatState)>)> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    heap.push(Reverse((0i16, start.0, start.1, CheatState::Cheatless)));
    visited.insert((start.0, start.1), (0i16, vec![]));

    let mut steps = 0i64;
    while let Some(Reverse((cost, x, y, c))) = heap.pop() {
        //if steps % 100000 == 0 {
        //dbg!(&(cost, x, y, c));
        //dbg!(&cost);
        //dbg!(heap.len());
        //dbg!(&cheat_entry);
        //}
        steps += 1;
        for (dx, dy) in [(0i16, 1i16), (1, 0), (-1, 0), (0, -1)] {
            //let old_c = c;
            let mut c = c;
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || nx >= grid[0].len() as i16 || ny < 0 || ny >= grid.len() as i16 {
                continue;
            }
            let neighbor = grid[ny as usize][nx as usize];
            if max_cost.is_none() && neighbor == '#' {
                continue;
            }
            if let Some(max_cost) = max_cost {
                if cost + 1 > max_cost {
                    continue;
                }
            }
            match (neighbor, c, cheat_entry) {
                ('#', CheatState::Cheatless, None) => {
                    if max_cheat_steps > 10 {
                        c = CheatState::CheatEnteredInfinite { enter: (nx, ny) }
                    } else {
                        c = CheatState::CheatEntered {
                            enter: (nx, ny),
                            steps: 1,
                        }
                    }
                }
                ('#', CheatState::Cheatless, Some(cheat)) => {
                    if (nx, ny) != cheat {
                        continue;
                    }
                    if max_cheat_steps > 10 {
                        c = CheatState::CheatEnteredInfinite { enter: (nx, ny) }
                    } else {
                        c = CheatState::CheatEntered {
                            enter: (nx, ny),
                            steps: 1,
                        }
                    }
                }
                ('#', CheatState::CheatEnteredInfinite { .. }, _) => {}
                ('#', CheatState::CheatEntered { enter, steps }, _) if steps < max_cheat_steps => {
                    c = CheatState::CheatEntered {
                        enter,
                        steps: steps + 1,
                    }
                }
                ('#', CheatState::CheatEntered { steps, .. }, _) if steps >= max_cheat_steps => {
                    continue;
                }
                ('#', CheatState::Cheated { .. }, _) => {
                    continue;
                }
                ('.', CheatState::CheatEntered { enter, steps }, _) if steps <= max_cheat_steps => {
                    if let Some(cheat_exit) = cheat_exit {
                        if cheat_exit != (nx, ny) {
                            continue;
                        }
                    }
                    c = CheatState::Cheated {
                        enter,
                        exit: (nx, ny),
                    };
                    if forbidden_cheats.contains(&c) {
                        continue;
                    }
                }
                ('.', CheatState::CheatEnteredInfinite { enter }, _) => {
                    c = CheatState::Cheated {
                        enter,
                        exit: (nx, ny),
                    };
                    if forbidden_cheats.contains(&c) {
                        continue;
                    }
                }
                ('.', CheatState::Cheatless | CheatState::Cheated { .. }, _) => {}
                (n, state, _) => panic!("Unexpected state: {n}, {state:?}"),
            };
            match visited.entry((nx, ny)) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    let (other_cost, p): &mut (i16, Vec<(i16, i16, CheatState)>) = o.get_mut();
                    if *other_cost == cost + 1 {
                        p.push((x, y, c));
                        //heap.push(Reverse((cost + 1, nx, ny, c)));
                    }
                    if *other_cost > cost + 1 {
                        *other_cost = cost + 1;
                        *p = vec![(x, y, c)];
                        heap.push(Reverse((cost + 1, nx, ny, c)));
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((cost + 1, vec![(x, y, c)]));
                    heap.push(Reverse((cost + 1, nx, ny, c)));
                }
            };
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
    print_grid(&grid, (1, 1, CheatState::Cheatless));

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

    {
        let max_cheat_steps = i16::MAX;
        let mut cheats = HashSet::new();
        let visited = do_search(&grid, start_pos, None, &cheats, None, None, max_cheat_steps);
        let cheatless = visited.get(&(end_pos.0, end_pos.1));
        dbg!(&cheatless);

        //let mut cheats = HashSet::new();
        //let total = wall_pos.len() * (wall_pos.len() - 1);
        //let mut count = 0;
        //for vec in repeat_n(wall_pos.iter(), 2).multi_cartesian_product() {
        //let a = vec[0];
        //let b = vec[1];
        //println!("{:.03} {count}/{total}", count as f64 / total as f64);
        //count += 1;
        //for y in 1..(grid.len() - 1) {
        ////println!("{y}/{}", grid.len());
        //for x in 1..(grid[0].len() - 1) {
        //println!("({x},{y})/{}", grid[0].len());
        //if grid[y][x] == '#' {

        let mut cont = true;
        while cont {
            dbg!(&cheats.len());
            let visited = do_search(
                &grid,
                start_pos,
                Some(cheatless.unwrap().0),
                &cheats,
                None,
                None,
                //Some(*a),
                //Some(*b),
                //Some((x as i16, y as i16)),
                max_cheat_steps,
            );

            cont = false;
            let mut todo = Vec::new();
            visited
                .iter()
                .filter(|(k, _)| (k.0, k.1) == (end_pos.0, end_pos.1))
                .filter(|&(_, v)| cheatless.unwrap().0 - v.0 >= 100)
                .for_each(|(k, v)| {
                    todo.clear();
                    todo.push(*k);
                    cont = true;
                    dbg!(&v.0, cheatless.unwrap().0);

                    while let Some(cur) = todo.pop() {
                        let prevs = &visited[&cur].1;
                        for &(x, y, c) in prevs {
                            if c.is_cheated() {
                                cheats.insert(c);
                            }
                            todo.push((x, y));
                        }
                    }
                });
        }
        //}
        //}
        //}
        //}
        //let part1 = cheats.len();
        //dbg!(&part1);
    }

    //dbg!(&cheats);
    Ok(())
}
