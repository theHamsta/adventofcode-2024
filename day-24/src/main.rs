#[allow(unused_imports)]
use itertools::Itertools;
use std::io::Write;

use rand::Rng;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
use std::fs::File;

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

fn solve(
    variables: &mut HashMap<String, bool>,
    equations: &[Equation],
    count_z: usize,
) -> Option<i64> {
    let values = variables;
    let max_shuffles = 100;
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

    //let count_z = values.keys().filter(|v| v.starts_with('z')).count();
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

fn calc_score(
    permutations: &Vec<(usize, usize)>,
    test_values: &Vec<(i64, i64, i64)>,
    variables: &HashMap<String, bool>,
    equations: &[Equation],
    counts: (usize, usize, usize),
) -> (bool, i64) {
    let mut mut_equations = equations.to_owned();
    let mut mut_variables = variables.clone();
    let (count_x, count_y, count_z) = counts;
    for &(a, b) in permutations.iter() {
        //if mut_equation[a].result > mut_equation[b].result {
        //return 0;
        //}
        let tmp = mut_equations[a].result.clone();
        mut_equations[a].result = mut_equations[b].result.clone();
        mut_equations[b].result = tmp;
    }

    let mut score = 0i64;
    let mut solved = true;
    for (x, y, z) in test_values.iter() {
        for i in 0..count_x {
            mut_variables.insert(format!("x{:02}", i), ((x >> i) & 1) != 0);
        }
        for i in 0..count_y {
            mut_variables.insert(format!("y{:02}", i), ((y >> i) & 1) != 0);
        }

        let my_z = solve(&mut mut_variables, &mut_equations, count_z);
        if my_z != Some(*z) {
            //dbg!(&"cont");
            solved = false;
        }
        if let Some(my_z) = my_z {
            let part_score = (!(my_z ^ z) & ((1 << count_z) - 1)).count_ones();
            //println!("{my_z:010b}");
            //println!("{z:010b}");
            //println!("{:010b}", !(my_z ^ z) & ((1 << count_z) - 1));
            //println!("{}", part_score);
            //println!();
            score += part_score as i64;
        } else {
            return (false, -1);
        }
    }
    (solved, score)
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

    let count_x = variables.keys().filter(|v| v.starts_with('x')).count();
    let count_y = variables.keys().filter(|v| v.starts_with('y')).count();
    let count_z = equations
        .iter()
        .map(|v| &v.result)
        .filter(|v| v.starts_with('z'))
        .count();
    let part1 = solve(&mut variables.clone(), &equations, count_z);
    dbg!(&part1);

    let mut test_values = Vec::new();
    for _ in 0..10 {
        let x = rng.gen_range(0..(1 << count_x));
        let y = rng.gen_range(0..(1 << count_y));
        let z = x + y;
        test_values.push((x, y, z));
    }
    dbg!(&test_values);

    let mut file = File::create("C:/tmp/foo.dot").unwrap();
    writeln!(file, "digraph {{").unwrap();
    for (i, eq) in equations.iter().enumerate() {
        let op_name = format!("{:?}_{i}", &eq.op);
        writeln!(file, "{} -> {op_name};", &eq.a).unwrap();
        writeln!(file, "{} -> {op_name};", &eq.b).unwrap();
        writeln!(file, "{op_name} -> {};", &eq.result).unwrap();
    }
    writeln!(file, "}}").unwrap();

    // Now analyzing the graphviz

    let vec = [
        ["gvw", "qjb"],
        ["z15", "jgc"],
        ["z22", "drg"],
        ["jbp", "z35"],
    ];

    let part2 = vec.iter().flatten().sorted().join(",");
    dbg!(&part2);

    Ok(())
}
