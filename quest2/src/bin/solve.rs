use std::{collections::HashMap, path::Path};

fn main() {
    let bone = load_file("input/everybody_codes_e3_q02_p1.txt");
    println!("part 1 = {}", quest2::part1::run(bone[0]));

    let bone = load_file("input/everybody_codes_e3_q02_p2.txt");
    println!("part 2 = {}", quest2::part2::run(&bone[0]));

    let bones = load_file("input/everybody_codes_e3_q02_p3.txt");
    println!("part 3 = {}", quest2::part3::run(bones));
}

fn load_file<P: AsRef<Path>>(path: P) -> Vec<(i32, i32)> {
    let data = std::fs::read_to_string(path).expect("file");

    let mut points = HashMap::<char, Vec<(i32, i32)>>::new();

    for (ch, point) in data
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, ch)| (ch, (r as i32, c as i32)))
        })
        .filter(|(ch, _)| *ch == '@' || *ch == '#')
    {
        let entry = points.entry(ch).or_default();
        entry.push(point);
    }

    let start = points.get(&'@').expect("missing starting location")[0];
    points
        .get(&'#')
        .expect("missing vocal bones location")
        .iter()
        .map(|bone| (bone.0 - start.0, bone.1 - start.1))
        .collect()
}
