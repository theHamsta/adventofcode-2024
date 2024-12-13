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

    let regex = Regex::new(r"(Button A|Button B|Prize):\s*X=?([-+]?\d+),\s*Y=?([-+]?\d+)").unwrap();

    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut prize = Vec::new();
    regex.captures_iter(raw_input).for_each(|c| match &c[1] {
        "Button A" => a.push((c[2].parse::<i64>().unwrap(), c[3].parse::<i64>().unwrap())),
        "Button B" => b.push((c[2].parse::<i64>().unwrap(), c[3].parse::<i64>().unwrap())),
        "Prize" => prize.push((c[2].parse::<i64>().unwrap(), c[3].parse::<i64>().unwrap())),
        _ => unreachable!(),
    });

    dbg!(&a);
    dbg!(&b);
    dbg!(&prize);

    let mut costs = Vec::new();

    for i in 0..a.len() {
        //let mut todo = BinaryHeap::new();
        //todo.push(Reverse((0i64, 0i64, 0i64)));

        //dbg!(&i);
        //let mut steps = 0;
        //while let Some(Reverse((cost, px, py))) = todo.pop() {
        //dbg!(&px,&py);
        //if px == prize[i] {
        //}
        //if (px, py) == prize[i] {
        //costs.push(cost);
        //break;
        //}
        //todo.push(Reverse((3 + cost, px + a[i].0, py + a[i].1)));
        //todo.push(Reverse((1 + cost, px + b[i].0, py + b[i].1)));
        //steps+=1;
        //if steps == 1000 {
        //break;
        //}
        //}
        costs.push(
            repeat_n(0i64..100, 2)
                .multi_cartesian_product()
                .filter_map(|vec| {
                    let na = vec[0];
                    let nb = vec[1];
                    //((na * a[i].0 + nb * b[i].0, na * a[i].1 + nb + b[i].1) == prize[i])
                    ((na * a[i].0 + nb * b[i].0, na * a[i].1 + nb * b[i].1) == prize[i])
                        .then_some((na, nb))
                })
                .map(|p| (3 * p.0 + p.1, p.0, p.1))
                .min_by_key(|p| p.0),
        );
    }

    let part1: i64 = costs.iter().filter_map(|c| c.and_then(|c| Some(c.0))).sum();
    dbg!(&part1);

    let prize = prize
        .iter()
        .map(|p| (p.0 + 10000000000000, p.1 + 10000000000000))
        .collect_vec();

    let mut costs = Vec::new();

    for i in 0..a.len() {
        let a = a[i];
        let b = b[i];
        let c = prize[i];
        let det = a.0 * b.1 - b.0 * a.1;
        if det == 0 {
            // actually never occurs :-(
            for steps_a in 0i64.. {
                let steps_b = (c.0 - a.0 * steps_a) / b.0;
                if steps_a * a.0 + steps_b * b.0 == c.0 && steps_a * a.1 + steps_b * b.1 == c.1 {
                    costs.push(Some(steps_a * 3 + steps_b));
                }
            }
        }
        let det_a = c.0 * b.1 - b.0 * c.1;
        let det_b = a.0 * c.1 - c.0 * a.1;

        let steps_a = det_a / det;
        let steps_b = det_b / det;
        if steps_a * a.0 + steps_b * b.0 == c.0 && steps_a * a.1 + steps_b * b.1 == c.1 {
            costs.push(Some(steps_a * 3 + steps_b));
        }
    }

    let part2: i64 = costs.iter().filter_map(|&c| c).sum();
    dbg!(&part2);

    Ok(())
}
