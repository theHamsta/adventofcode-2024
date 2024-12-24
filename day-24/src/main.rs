#[allow(unused_imports)]
use itertools::Itertools;

use rand::Rng;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::thread_rng;

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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Equation {
    a: String,
    b: String,
    result: String,
    op: Op,
}

fn solve(variables: &mut HashMap<String, bool>, equations: &[Equation]) -> Option<i64> {
    let values = variables;
    let max_shuffles = 10;
    let mut shuffles = 0;

    let mut todo_eqn = equations.to_owned();
    while let Some(eq) = todo_eqn.pop() {
        match (eq.op, values.get(&eq.a), values.get(&eq.b)) {
            (Op::And, Some(a), Some(b)) => values.insert(eq.result, a & b),
            (Op::Or, Some(a), Some(b)) => values.insert(eq.result, a | b),
            (Op::Xor, Some(a), Some(b)) => values.insert(eq.result, a ^ b),
            _ => {
                if shuffles >= max_shuffles {
                    return None;
                }
                //dbg!(&eq);
                todo_eqn.push(eq);
                // poor man's topo sort
                todo_eqn.shuffle(&mut thread_rng());
                shuffles += 1;

                None
            }
        };
    }
    //dbg!(&values);

    let count_z = values.keys().filter(|v| v.starts_with('z')).count();
    Some(
        (0..count_z)
            .flat_map(|i| {
                values
                    .get(&format!("z{:02}", i))
                    //.inspect(|v| {
                    //dbg!(format!("z{:02}", i), v);
                    //})
                    .map(|v| *v as i64 * (1 << i as i64))
            })
            .sum(),
    )
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example2");

    //let raw_input = include_str!("../example");
    //let raw_input = include_str!("../example3");

    let var_regex = Regex::new(r"(\w+):\s+(\d+)").unwrap();
    let eq_regex = Regex::new(r"(\w+)\s+(\w+)\s+(\w+)\s+[-]>\s+(\w+)").unwrap();

    let variables = var_regex
        .captures_iter(raw_input)
        .map(|c| (c[1].to_owned(), c[2].parse::<i64>().unwrap() != 0))
        .collect::<HashMap<_, _>>();
    //dbg!(&variables);
    let equations = eq_regex
        .captures_iter(raw_input)
        .map(|c| Equation {
            a: c[1].to_owned(),
            op: match &c[2] {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                op => panic!("Unexpected operator: {op}"),
            },
            b: c[3].to_owned(),
            result: c[4].to_owned(),
        })
        .collect_vec();
    //dbg!(&equations);

    let mut rng = rand::thread_rng();

    let part1 = solve(&mut variables.clone(), &equations);
    dbg!(&part1);

    let count_x = variables.keys().filter(|v| v.starts_with('x')).count();
    let count_y = variables.keys().filter(|v| v.starts_with('y')).count();

    let swapped_pairs = 4;
    let total = equations.iter().permutations(swapped_pairs * 2).count();
    let mut test_values = Vec::new();
    for _ in 0..10 {
        let x = rng.gen_range(0..(1 << count_x));
        let y = rng.gen_range(0..(1 << count_y));
        let z = x + y;
        test_values.push((x, y, z));
    }
    dbg!(&test_values);

    let mut mut_equation = equations.clone();
    let mut mut_variables = variables.clone();
    let eqn_len = equations.len();
    'outer: for (i, c) in (0..eqn_len).permutations(swapped_pairs * 2).enumerate() {
        //println!("{i}/{total}");
        if i % 10000 == 0 {
            println!("{i}/{total} {:.04}", i as f64 / total as f64);
        }
        mut_equation.clear();
        mut_equation = equations.clone();
        for pair in c.chunks(2) {
            if mut_equation[pair[0]].result > mut_equation[pair[1]].result {
                continue 'outer;
            }
            let tmp = mut_equation[pair[0]].result.clone();
            mut_equation[pair[0]].result = mut_equation[pair[1]].result.clone();
            mut_equation[pair[1]].result = tmp;
        }

        for (x, y, z) in test_values.iter() {
            for i in 0..count_x {
                mut_variables.insert(format!("x{:02}", i), ((x >> i) & 1) != 0);
            }
            for i in 0..count_x {
                mut_variables.insert(format!("y{:02}", i), ((y >> i) & 1) != 0);
            }

            let my_z = solve(&mut mut_variables, &mut_equation);
            if my_z != Some(*z) {
                //dbg!(&"cont");
                continue 'outer;
            }
        }
        dbg!(&"found");

        let part2 = c
            .iter()
            .map(|e| equations[*e].result.clone())
            .sorted()
            .join(",");
        dbg!(&part2);
        return Ok(());
    }

    Ok(())
}
