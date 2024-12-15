#![feature(hash_set_entry)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
use std::process::exit;

fn print_grid(grid: &[Vec<char>], pos: (i64, i64)) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if (x as i64, y as i64) == pos {
                print!("*");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
    println!();
}

fn solve(board: &[Vec<char>], movement: &str, debug: bool) -> i64 {
    let mut board = board.to_owned();
    let mut start_pos = (0i64, 0i64);

    'outer: for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == '@' {
                start_pos = (x as i64, y as i64);
                break 'outer;
            }
        }
    }

    //print_grid(&board, start_pos);
    let mut orig_count = 0i64;
    if debug {
        for y in 0..board.len() {
            for x in 0..board[0].len() {
                if board[y][x] == '[' || board[y][x] == 'O' {
                    orig_count += 1;
                }
            }
        }
    }

    let mut pos = start_pos;
    for (step, mov) in movement.chars().enumerate() {
        let dir = match mov {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '\n' => continue,
            c => panic!("Reached unexpected char {c:?}"),
        };

        let neighbor = (pos.0 + dir.0, pos.1 + dir.1);
        match board[neighbor.1 as usize][neighbor.0 as usize] {
            '#' => continue,
            '.' | '@' => pos = neighbor,
            'O' | '[' | ']' => {
                let copy = board.clone();

                let mut to_push = vec![neighbor];
                let mut pushed = HashSet::new();
                let mut move_canceled = false;
                while let Some(b) = to_push.pop() {
                    let c = copy[b.1 as usize][b.0 as usize];
                    if !(c == 'O' || c == '[' || c == ']') {
                        continue;
                    }
                    if pushed.contains(&b) {
                        continue;
                    } else {
                        pushed.insert(b);
                    }

                    let to_overwrite = copy[(b.1 + dir.1) as usize][(b.0 + dir.0) as usize];
                    if to_overwrite == '#' {
                        // cancel move, move is blocked
                        board = copy;
                        move_canceled = true;
                        break;
                    }
                    board[(b.1 + dir.1) as usize][(b.0 + dir.0) as usize] =
                        copy[b.1 as usize][b.0 as usize];
                    if !pushed.contains(&(b.0 - dir.0, b.1 - dir.1)) {
                        board[b.1 as usize][b.0 as usize] = '.';
                    }

                    match copy[b.1 as usize][b.0 as usize] {
                        'O' => {
                            to_push.push((b.0 + dir.0, b.1 + dir.1));
                        }
                        '[' => {
                            to_push.push((b.0 + dir.0, b.1 + dir.1));
                            to_push.push((b.0 + 1, b.1));
                        }
                        ']' => {
                            to_push.push((b.0 + dir.0, b.1 + dir.1));
                            to_push.push((b.0 - 1, b.1));
                        }
                        _ => (),
                    }
                }
                if !move_canceled {
                    pos = neighbor;
                }
            }
            c => panic!("Reached unexpected char {c:?}"),
        }
        if debug {
            let mut count = 0i64;
            for y in 0..board.len() {
                for x in 0..board[0].len() {
                    if board[y][x] == '[' || board[y][x] == 'O' {
                        count += 1;
                    }
                }
            }
            if count != orig_count {
                dbg!(&count);
                println!("{} {}/{}", mov, step, movement.len());
                print_grid(&board, pos);
                exit(1);
            }
            //println!("{} {}/{}", mov, step, movement.len());
            //print_grid(&board, pos);
        }
    }
    //print_grid(&board, pos);
    let mut sum: i64 = 0;

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == '[' || board[y][x] == 'O' {
                sum += x as i64 + 100 * y as i64;
            }
        }
    }
    sum
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    let mut it = raw_input.split("\n\n");
    let board = it.next().unwrap();
    let movement = it.next().unwrap();
    //dbg!(&board);
    //dbg!(&movement);

    let board = board
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();

    let debug = false;
    let part1 = solve(&board, movement, debug);
    dbg!(&part1);

    let board = board
        .iter()
        .map(|l| {
            l.iter()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    '@' => ['@', '.'],
                    '.' => ['.', '.'],
                    'O' => ['[', ']'],
                    c => panic!("Reached unexpected char {c:?}"),
                })
                .collect_vec()
        })
        .collect_vec();

    let part2 = solve(&board, movement, debug);
    dbg!(&part2);

    Ok(())
}
