use std::collections::HashMap;

mod part1;

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e3_q02_p1.txt").expect("file");
    // let data = std::fs::read_to_string("input/test-part-1.txt").expect("file");

    let points: HashMap<_, _> = data
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, ch)| (ch, (r as i32, c as i32)))
        })
        .filter(|(ch, _)| *ch == '@' || *ch == '#')
        .collect(); // Then extract them from the collection
    let start = points.get(&'@').unwrap_or(&(0, 0));
    let bone = points.get(&'#').unwrap_or(&(0, 0));

    let bone = (bone.0 - start.0, bone.1 - start.1);
    println!("part 1 = {}", crate::part1::run(&bone));
}
