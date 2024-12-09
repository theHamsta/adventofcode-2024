//#![feature(array_windows)]
#![feature(new_range_api)]
use core::range::Range;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct File {
    orig_range: Range<usize>,
    compacted: usize,
    compacted_positions: Vec<Range<usize>>,
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example");

    let mut files = Vec::new();
    let mut spaces = Vec::new();
    raw_input.lines().filter(|l| !l.is_empty()).for_each(|l| {
        let mut chars = l.chars();
        let mut pos = 0usize;
        while let (Some(file), space) = (chars.next(), chars.next()) {
            let file = file as usize - '0' as usize;
            files.push(File {
                orig_range: (pos..(pos + file)).into(),
                compacted: 0,
                compacted_positions: Vec::new(),
            });
            pos += file;
            if let Some(space) = space {
                let space = space as usize - '0' as usize;
                spaces.push(pos..(pos + space));
                pos += space;
            }
        }
    });
    for is_part2 in [false, true] {
        let mut files = files.clone();
        for space in spaces.iter_mut() {
            let mut file_pos = files.len() - 1;
            let mut space_size = space.len();
            while space_size > 0 {
                let last_file = &mut files[file_pos];
                let file_size = last_file.orig_range.iter().len();
                let to_compact = file_size - last_file.compacted;

                if space.start > last_file.orig_range.start {
                    break;
                }

                let move_size = space_size.min(to_compact);
                if is_part2 && move_size < file_size {
                    if file_pos > 0 {
                        file_pos -= 1;
                    } else {
                        break;
                    }
                    continue;
                }

                last_file.compacted += move_size;
                last_file
                    .compacted_positions
                    .push((space.start..(space.start + move_size)).into());

                if last_file.compacted == file_size && file_pos > 0 {
                    file_pos -= 1;
                }

                space.start += move_size;
                space_size = space.len();
            }
        }
        //dbg!(&files);

        //dbg!(&spaces);
        let solution: i64 = files
            .iter()
            .enumerate()
            .map(|(id, f)| {
                let mut sum = 0i64;
                let uncompacted = f.orig_range.start..(f.orig_range.end - f.compacted);
                assert_eq!(uncompacted.len() + f.compacted, f.orig_range.iter().len());
                for pos in uncompacted {
                    sum += (id * pos) as i64;
                }
                for r in &f.compacted_positions {
                    for pos in r.iter() {
                        sum += (id * pos) as i64;
                    }
                }
                sum
            })
            .sum();

        dbg!(&solution);
    }

    //let regex = Regex::new(
    //r"([-]?\d+),\s*([-]?\d+),\s*([-]?\d+)\s*@\s*([-]?\d+),\s*([-]?\d+),\s*([-]?\d+)",
    //)
    //.unwrap();

    //let part1 = count_antinodes(&grid, &antenas, false);
    //dbg!(part1);

    //let part2 = count_antinodes(&grid, &antenas, true);
    //dbg!(part2);

    Ok(())
}
