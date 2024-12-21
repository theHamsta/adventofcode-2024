#[allow(unused_imports)]
use itertools::Itertools;
use num_traits::sign::signum;

#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
use std::iter::repeat_n;

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

fn translate(numbers: &[u8], state_table: &HashMap<(u8, u8), Vec<u8>>) -> Vec<u8> {
    let mut rtn = vec![];
    rtn.extend(state_table[&(b'A', numbers[0])].iter());
    rtn.push(b'A');

    for w in numbers.windows(2) {
        let a = w[0];
        let b = w[1];
        rtn.extend(state_table[&(a, b)].iter());
        rtn.push(b'A');
    }
    rtn
}

fn calc_state_table(keyboard: &HashMap<u8, (i32, i32)>) -> HashMap<(u8, u8), Vec<u8>> {
    let mut num_to_dir = HashMap::new();
    let keyboard_locations: HashSet<_> = keyboard.values().collect();

    for v in repeat_n(keyboard.keys(), 2).multi_cartesian_product() {
        let a = v[0];
        let b = v[1];
        let (ay, ax) = keyboard[a];
        let (by, bx) = keyboard[b];

        let (dy, dx) = (by - ay, bx - ax);

        let mut path = Vec::new();

        for y in 1i32..=dy.abs() {
            let (nx, ny) = (ax, ay + y * dy.signum());
            if keyboard_locations.contains(&(ny, nx)) {
                path.push(if dy.signum() == 1 { b'v' } else { b'^' });
            }
        }
        for x in 1i32..=dx.abs() {
            let (nx, ny) = (ax + x * dx.signum(), by);
            if keyboard_locations.contains(&(ny, nx)) {
                path.push(if dx.signum() == 1 { b'>' } else { b'<' });
            }
        }
        if path.len() as i32 != (dx.abs() + dy.abs()) {
            path.clear();
            for x in 1..=dx.abs() {
                let (nx, ny) = (ax + x * dx.signum(), ay);
                if keyboard_locations.contains(&(ny, nx)) {
                    path.push(if dx.signum() == 1 { b'>' } else { b'<' });
                }
            }
            for y in 1..=dy.abs() {
                let (nx, ny) = (bx, ay + y * dy.signum());
                if keyboard_locations.contains(&(ny, nx)) {
                    path.push(if dy.signum() == 1 { b'v' } else { b'^' });
                }
            }
        }
        //dbg!(*a as char);
        //dbg!(*b as char);
        //dbg!(&String::from_utf8(path.clone()).unwrap());
        //dbg!(&dx, dy);
        //assert_eq!(path.len() as i32, dx.abs() + dy.abs());
        num_to_dir.insert((*a, *b), path);
    }
    num_to_dir
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    let raw_input = include_str!("../example");

    //let raw_input = include_str!("../example");

    let input = raw_input.lines().filter(|l| !l.is_empty()).collect_vec();

    let mut numeric_keyboard = HashMap::new();
    numeric_keyboard.insert(b'1', (2, 0));
    numeric_keyboard.insert(b'2', (2, 1));
    numeric_keyboard.insert(b'3', (2, 2));

    numeric_keyboard.insert(b'4', (1, 0));
    numeric_keyboard.insert(b'5', (1, 1));
    numeric_keyboard.insert(b'6', (1, 2));

    numeric_keyboard.insert(b'7', (0, 0));
    numeric_keyboard.insert(b'8', (0, 1));
    numeric_keyboard.insert(b'9', (0, 2));

    numeric_keyboard.insert(b'0', (3, 1));
    numeric_keyboard.insert(b'A', (3, 2));

    let mut dir_keyboard = HashMap::new();
    dir_keyboard.insert(b'^', (0, 1));
    dir_keyboard.insert(b'A', (0, 2));
    dir_keyboard.insert(b'<', (1, 0));
    dir_keyboard.insert(b'v', (1, 1));
    dir_keyboard.insert(b'>', (1, 2));

    let num_to_dir = calc_state_table(&numeric_keyboard);
    let dir_to_dir = calc_state_table(&dir_keyboard);

    let mut sum = 0i64;
    for seq in input.iter() {
        let num = translate(seq.as_bytes(), &num_to_dir);
        let num = translate(&num, &dir_to_dir);
        let num = translate(&num, &dir_to_dir);
        let num_part: i64 =
            String::from_utf8(seq.as_bytes()[0..(seq.len() - 1)].to_owned())
                .unwrap()
                .parse()
                .unwrap();
        dbg!(&num.len());
        dbg!(&num_part);
        sum += num_part * num.len() as i64;
        println!(
            "{seq}: {} ({})",
            String::from_utf8(num.clone()).unwrap(),
            num.len()
        );
    }
    let part1 = sum;
    dbg!(&part1);

    Ok(())
}
