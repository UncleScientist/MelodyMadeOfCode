use std::{collections::HashMap, path::Path};

mod part1;
mod part2;

fn main() {
    let bone = load_file("input/everybody_codes_e3_q02_p1.txt");
    println!("part 1 = {}", crate::part1::run(&bone));

    let bone = load_file("input/everybody_codes_e3_q02_p2.txt");
    // let bone = load_file("input/test-part-2.txt");
    println!("part 2 = {}", crate::part2::run(&bone));
}

fn load_file<P: AsRef<Path>>(path: P) -> (i32, i32) {
    let data = std::fs::read_to_string(path).expect("file");

    let points: HashMap<_, _> = data
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, ch)| (ch, (r as i32, c as i32)))
        })
        .filter(|(ch, _)| *ch == '@' || *ch == '#')
        .collect();
    let start = points.get(&'@').unwrap_or(&(0, 0));
    let bone = points.get(&'#').unwrap_or(&(0, 0));

    (bone.0 - start.0, bone.1 - start.1)
}
