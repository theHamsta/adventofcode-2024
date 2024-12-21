#[allow(unused_imports)]
use itertools::Itertools;

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

#[allow(dead_code)]
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

fn translate2(
    state: &[u8],
    numbers: &[u8],
    state_table: &HashMap<(u8, u8), Vec<Vec<u8>>>,
    recursion_state_table: &HashMap<(u8, u8), Vec<Vec<u8>>>,
    recursions: u32,
    cost_cache: &mut HashMap<(Vec<u8>, Vec<u8>, u32), (i64, Vec<u8>)>,
) -> (i64, Vec<u8>) {
    if numbers.is_empty() {
        //return (0i64, state.to_vec(), vec![]);
        return (0i64, state.to_vec());
    }
    let old_state = state;
    //128962
    if let Some(rtn) = cost_cache.get(&(old_state.to_owned(), numbers.to_owned(), recursions)) {
        return rtn.clone();
    }

    //dbg!(&String::from_utf8(state.to_vec()).unwrap());
    //dbg!(&String::from_utf8(numbers.to_vec()).unwrap());
    //dbg!(state[0] as char);
    //dbg!(numbers[0] as char);
    //dbg!(&recursions);
    let transitions = &state_table[&(state[0], numbers[0])];
    let (cost, state) = transitions
        .iter()
        .map(|transition| {
            {
                let (cost, state) = if recursions == 0 {
                    (transition.len() as i64, vec![])
                } else {
                    translate2(
                        &state[1..],
                        transition,
                        recursion_state_table,
                        recursion_state_table,
                        recursions - 1,
                        cost_cache,
                    )
                };

                //dbg!(&state);
                assert!(state.len() as u32 == recursions);
                let mut new_state = vec![numbers[0]];
                new_state.extend(&state);
                let (new_cost, new_state) = translate2(
                    &new_state,
                    &numbers[1..],
                    state_table,
                    recursion_state_table,
                    recursions,
                    cost_cache,
                );
                //dbg!(&new_state);
                //dbg!(&recursions);
                assert!(new_state.len() as u32 == recursions + 1);
                //trans.extend(new_trans);
                (cost + new_cost, new_state)
            }
        })
        .min_by_key(|(c, _)| *c)
        .unwrap();
    //dbg!(&state);
    //dbg!(&numbers);
    cost_cache.insert(
        (old_state.to_owned(), numbers.to_owned(), recursions),
        (cost, state.clone()),
    );
    assert!(state.len() as u32 == recursions + 1);
    (cost, state)
}

fn calc_state_table(keyboard: &HashMap<u8, (i32, i32)>) -> HashMap<(u8, u8), Vec<Vec<u8>>> {
    let mut num_to_dir = HashMap::new();
    let keyboard_locations: HashSet<_> = keyboard.values().collect();

    for v in repeat_n(keyboard.keys(), 2).multi_cartesian_product() {
        let a = v[0];
        let b = v[1];
        let (ay, ax) = keyboard[a];
        let (by, bx) = keyboard[b];

        let (dy, dx) = (by - ay, bx - ax);

        let mut paths = Vec::new();

        let dist = dx.abs() + dy.abs();
        for v in repeat_n(0, dx.unsigned_abs() as usize)
            .chain(repeat_n(1, dy.unsigned_abs() as usize))
            .permutations(dist as usize)
        {
            let mut path = Vec::new();
            let (mut ny, mut nx) = (ay, ax);
            for &n in v.iter() {
                if n == 0 {
                    nx += dx.signum();
                    if keyboard_locations.contains(&(ny, nx)) {
                        path.push(if dx.signum() == 1 { b'>' } else { b'<' });
                    }
                } else {
                    ny += dy.signum();
                    if keyboard_locations.contains(&(ny, nx)) {
                        path.push(if dy.signum() == 1 { b'v' } else { b'^' });
                    }
                }
            }
            if path.len() as i32 == dist {
                path.push(b'A');
                paths.push(path);
            }
        }

        num_to_dir.insert((*a, *b), paths);
    }
    num_to_dir
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example");

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

    let mut cache = HashMap::new();
    let ground_truth = b"<A^A^^>AvvvA".len();
    let (cost, _state) = translate2(b"A", b"029A", &num_to_dir, &dir_to_dir, 0, &mut cache);
    assert_eq!(cost, ground_truth as i64);
    let ground_truth = b"v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len();
    let (cost, _state) = translate2(b"AA", b"029A", &num_to_dir, &dir_to_dir, 1, &mut cache);
    assert_eq!(cost, ground_truth as i64);

    let mut sum = 0i64;
    let mut cache = HashMap::new();
    for num_recursions in [2, 25] {
        for seq in input.iter() {
            let (cost, _) = translate2(
                &b"AAAAAAAAAAAAAAAAAAAAAAAAAA"[..=(num_recursions as usize)],
                seq.as_bytes(),
                &num_to_dir,
                &dir_to_dir,
                num_recursions,
                &mut cache,
            );
            let num_part: i64 = String::from_utf8(seq.as_bytes()[0..(seq.len() - 1)].to_owned())
                .unwrap()
                .parse()
                .unwrap();
            //dbg!(&num_part);
            //dbg!(&cost);
            sum += num_part * cost;
            //println!(
            //"{seq}: {} ({})",
            //String::from_utf8(num.clone()).unwrap(),
            //num.len()
            //);
        }
        let part1 = sum;
        dbg!(&part1);
    }

    Ok(())
}
