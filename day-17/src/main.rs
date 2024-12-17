use std::collections::HashSet;

use log::debug;
use log::info;
use regex::Regex;

fn decode_operand(registers: &[i64; 3], operand: i64) -> i64 {
    match operand {
        c @ 0..=3 => c,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!(),
    }
}

fn decode_human(operand: i64) -> String {
    match operand {
        c @ 0..=3 => format!("{c}"),
        4 => "A".to_owned(),
        5 => "B".to_owned(),
        6 => "C".to_owned(),
        _ => panic!(),
    }
}

#[derive(Debug)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<i64> for Op {
    fn from(value: i64) -> Self {
        match value {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            7 => Op::Cdv,
            inst => panic!("Unknown instruction {inst}"),
        }
    }
}

fn run(registers: &[i64; 3], program: &[i64]) -> usize {
    let mut cur_regs = registers.to_owned();

    let mut matched = 0;
    let mut ip = 0;
    while ip < program.len() {
        let op = program[ip];
        let operand = program[ip + 1];
        let op = Op::from(op);
        //if !seen_states.insert((cur_regs[0], cur_regs[1], cur_regs[2], ip, output.clone())) {
        //return false;
        //}
        ip += 2;

        //debug!(
        //"A: {}, B: {}, C: {}, {ip:?}",
        //cur_regs[0], cur_regs[1], cur_regs[2]
        //);
        //debug!("{op:?} {operand}",);
        match op {
            Op::Adv => cur_regs[0] /= 1 << decode_operand(&cur_regs, operand),
            Op::Bxl => cur_regs[1] ^= operand,
            Op::Bst => cur_regs[1] = decode_operand(&cur_regs, operand) % 8,
            Op::Jnz => {
                if cur_regs[0] != 0 {
                    ip = operand as usize;
                }
            }
            Op::Bxc => cur_regs[1] ^= cur_regs[2],
            Op::Out => {
                let output = decode_operand(&cur_regs, operand) % 8;
                if matched >= program.len() {
                    return 0;
                }
                if output == program[matched] {
                    matched += 1;
                } else {
                    return matched;
                }
            }
            Op::Bdv => cur_regs[1] = cur_regs[0] / (1 << decode_operand(&cur_regs, operand)),
            Op::Cdv => cur_regs[2] = cur_regs[0] / (1 << decode_operand(&cur_regs, operand)),
        }
    }
    matched
}

fn main() {
    pretty_env_logger::init();
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example3");

    let regex = Regex::new(r"Register (A|B|C): ([-]?\d+)").unwrap();
    let program = Regex::new(r"Program: (.+)").unwrap();

    let mut registers = [0i64; 3];
    for c in regex.captures_iter(raw_input) {
        match &c[1] {
            "A" => registers[0] = c[2].parse().unwrap(),
            "B" => registers[1] = c[2].parse().unwrap(),
            "C" => registers[2] = c[2].parse().unwrap(),
            _ => panic!(),
        }
    }
    dbg!(&registers);

    let program = program.captures(raw_input).unwrap();
    let program: Vec<i64> = program[1]
        .split(",")
        .map(|i| i.trim().parse().unwrap())
        .collect();
    info!("Disassembly");
    for v in program.chunks(2) {
        let op = Op::from(v[0]);
        let operand = v[1];

        match op {
            Op::Bxl | Op::Jnz => info!("{op:?} {operand}"),
            Op::Bxc => info!("{op:?}"),
            op => info!("{op:?} {}", decode_human(operand)),
        }
    }

    dbg!(&program.len());

    let mut cur_regs = registers.clone();
    let mut ip = 0;
    let mut output = Vec::new();

    while ip < program.len() {
        let op = program[ip];
        let operand = program[ip + 1];
        let op = Op::from(op);
        ip += 2;

        debug!(
            "A: {}, B: {}, C: {}, {ip:?}",
            cur_regs[0], cur_regs[1], cur_regs[2]
        );
        debug!("{op:?} {operand}",);
        match op {
            Op::Adv => cur_regs[0] /= 1 << decode_operand(&cur_regs, operand),
            Op::Bxl => cur_regs[1] ^= operand,
            Op::Bst => cur_regs[1] = decode_operand(&cur_regs, operand) % 8,
            Op::Jnz => {
                if cur_regs[0] != 0 {
                    ip = operand as usize;
                }
            }
            Op::Bxc => cur_regs[1] ^= cur_regs[2],
            Op::Out => {
                output.push(decode_operand(&cur_regs, operand) % 8);
                debug!("Output: {output:?}");
            }
            Op::Bdv => cur_regs[1] = cur_regs[0] / (1 << decode_operand(&cur_regs, operand)),
            Op::Cdv => cur_regs[2] = cur_regs[0] / (1 << decode_operand(&cur_regs, operand)),
        }
    }
    dbg!(&cur_regs);
    dbg!(&output);

    let part1: String = output.iter().map(|i| format!("{i},")).collect();
    let part1 = &part1[0..part1.len() - 1];
    dbg!(&part1);

    let mut candidates = Vec::new();
    for i in 1..=program.len() {
        if candidates.is_empty() {
            // in disassembly we can see that only the least significant 16 bits of A are used,
            // then A /= 8
            candidates = (0..(1 << 16))
                .filter(|&c| run(&[c, registers[1], registers[2]], &program) >= i)
                .collect();
        }
        candidates = candidates
            .iter()
            .flat_map(|c| {
                [
                    c | (0 << (16 + (i - 1) * 3)),
                    c | (1 << (16 + (i - 1) * 3)),
                    c | (2 << (16 + (i - 1) * 3)),
                    c | (3 << (16 + (i - 1) * 3)),
                    c | (4 << (16 + (i - 1) * 3)),
                    c | (5 << (16 + (i - 1) * 3)),
                    c | (6 << (16 + (i - 1) * 3)),
                    c | (7 << (16 + (i - 1) * 3)),
                ]
            })
            .filter(|&c| run(&[c, registers[1], registers[2]], &program) >= i && i <= program.len())
            .collect();
    }
    candidates.sort();
    let part2 = candidates[0];
    dbg!(&part2);
    dbg!(&candidates.len());
    //34681347993277

    //dbg!(&part2);
}
