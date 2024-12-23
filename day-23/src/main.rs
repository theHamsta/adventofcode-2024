#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
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

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example");

    let input = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut it = l.split('-');
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect_vec();

    let mut graph = HashMap::new();

    for (a, b) in input {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let mut three_cliques = HashSet::new();

    let mut count = 0i64;
    for (a, b, c) in graph.keys().tuple_combinations() {
        if graph[a].contains(b) && graph[a].contains(c) && graph[b].contains(c) {
            if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
                count += 1;
            }
            three_cliques.insert((a, b, c));
        }
    }
    let part1 = count;
    dbg!(&part1);

    let max_clique = three_cliques
        .iter()
        .map(|c| {
            let mut nodes = HashSet::new();
            nodes.insert(c.0);
            nodes.insert(c.1);
            nodes.insert(c.2);

            for node in graph.keys() {
                if nodes.iter().all(|n| graph[*n].contains(node)) {
                    nodes.insert(node);
                }
            }
            let len = nodes.len();
            (nodes, len)
        })
        //.collect_vec();
        .max_by_key(|n| n.1);
    let max_clique = max_clique.unwrap().0;
    let part2 = max_clique.iter().sorted().join(",");
    dbg!(&part2);

    Ok(())
}
