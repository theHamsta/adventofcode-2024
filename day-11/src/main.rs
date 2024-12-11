#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::mem::swap;

fn split(x: i64) -> (i64, i64) {
    let digits = x.ilog10() + 1;
    let a = x / 10i64.pow(digits / 2);
    let b = x % 10i64.pow(digits / 2);
    (a, b)
}

fn calc(input_num: i64, steps: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if steps == 0 {
        return 1;
    }
    let result = cache.get(&(input_num, steps));
    if let Some(result) = result {
        return *result;
    }

    let result = match input_num {
        0 => calc(1, steps - 1, cache),
        n if (n.ilog10() + 1) % 2 == 0 => {
            let (a, b) = split(n);
            calc(a, steps - 1, cache) + calc(b, steps - 1, cache)
        }
        n => calc(n * 2024, steps - 1, cache),
    };

    cache.insert((input_num, steps), result);
    result
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example");

    let numbers = raw_input
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec();

    let mut cur = numbers.clone();
    let mut next = Vec::new();

    //print_grid(&grid);

    let num_steps = 6;

    for _i in 0..num_steps {
        next.clear();
        for n in cur.iter() {
            match n {
                0 => {
                    next.push(1);
                }
                n if (n.ilog10() + 1) % 2 == 0 => {
                    let (a, b) = split(*n);
                    next.push(a);
                    next.push(b);
                }
                n => {
                    next.push(n * 2024);
                }
            }
        }

        //dbg!(_i, &next, next.len());
        swap(&mut cur, &mut next);
    }
    dbg!(&cur.len());

    let steps = 75;
    let mut cache = HashMap::new();
    let part2: i64 = numbers.iter().map(|&n| calc(n, steps, &mut cache)).sum();
    dbg!(&part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_name_test() {
        assert_eq!(split(1234), (12, 34));
    }
}
