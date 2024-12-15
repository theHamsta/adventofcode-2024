#![feature(hash_set_entry)]
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
use std::thread::sleep_ms;

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

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    let mut it = raw_input.split("\n\n");
    let board = it.next().unwrap();
    let movement = it.next().unwrap();
    //dbg!(&board);
    //dbg!(&movement);

    let input = board
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();

    let mut board = input.clone();
    let mut start_pos = (0i64, 0i64);

    'outer: for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == '@' {
                start_pos = (x as i64, y as i64);
                break 'outer;
            }
        }
    }
    //print_grid(&input, start_pos);

    let mut pos = start_pos;
    for mov in movement.chars() {
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
            'O' => {
                for i in 2.. {
                    let after_rock = (pos.0 + i * dir.0, pos.1 + i * dir.1);
                    match board[after_rock.1 as usize][after_rock.0 as usize] {
                        '.' | '@' => {
                            pos = neighbor;
                            board[after_rock.1 as usize][after_rock.0 as usize] = 'O';
                            board[neighbor.1 as usize][neighbor.0 as usize] = '.';
                            break;
                        }
                        'O' => {
                            continue;
                        }
                        '#' => break,
                        c => panic!("Reached unexpected char {c:?}"),
                    }
                }
            }
            c => panic!("Reached unexpected char {c:?}"),
        }
        //println!("{}", mov);
        //print_grid(&board, pos);
    }

    let mut part1: i64 = 0;

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == 'O' {
                part1 += x as i64 + 100 * y as i64;
            }
        }
    }
    dbg!(&part1);

    let mut board = input
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

    'outer: for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == '@' {
                start_pos = (x as i64, y as i64);
                break 'outer;
            }
        }
    }
    //print_grid(&board, start_pos);

    let mut pos = start_pos;
    for mov in movement.chars() {
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
                for i in 2.. {
                    let after_rock = (pos.0 + i * dir.0, pos.1 + i * dir.1);
                    let copy = board.clone();
                    let copy_pos = pos;
                    match board[after_rock.1 as usize][after_rock.0 as usize] {
                        '.' | '@' => {
                            pos = neighbor;

                            let mut to_push = vec![neighbor];
                            let mut pushed = HashSet::new();
                            while let Some(b) = to_push.pop() {
                                if pushed.contains(&b) {
                                    continue;
                                } else {
                                    pushed.insert(b);
                                }

                                let c = copy[b.1 as usize][b.0 as usize];
                                if !(c == 'O' || c == '[' || c == ']') {
                                    continue;
                                }
                                let to_overwrite =
                                    copy[(b.1 + dir.1) as usize][(b.0 + dir.0) as usize];
                                if to_overwrite == '#' {
                                    // cancel move, move is blocked
                                    board = copy;
                                    pos = copy_pos;
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
                                        if dir != (1, 0) {
                                            to_push.push((b.0 + 1, b.1));
                                        }
                                    }
                                    ']' => {
                                        to_push.push((b.0 + dir.0, b.1 + dir.1));
                                        if dir != (-1, 0) {
                                            to_push.push((b.0 - 1, b.1));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            break;
                        }
                        'O' | '[' | ']' => {
                            continue;
                        }
                        '#' => break,
                        c => panic!("Reached unexpected char {c:?}"),
                    }
                }
            }
            c => panic!("Reached unexpected char {c:?}"),
        }
        //println!("{}", mov);
        //print_grid(&board, pos);
        //sleep_ms(250);
    }
    //print_grid(&board, pos);
    let mut part2: i64 = 0;

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == '['  {
                part2 += x as i64 + 100 * y as i64;
            }
        }
    }
    dbg!(&part2);

    Ok(())
}
