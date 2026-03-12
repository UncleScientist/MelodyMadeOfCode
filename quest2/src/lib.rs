use std::{
    collections::{HashMap, hash_set::Iter},
    path::Path,
};

pub mod part1;
pub mod part2;
pub mod part3;

pub fn load_file<P: AsRef<Path>>(path: P) -> Vec<(i32, i32)> {
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

pub struct DrawState<'a> {
    pub bounding_box: (i32, i32, i32, i32),
    pub visited: Iter<'a, (i32, i32)>,
    pub cur_loc: (i32, i32),
    pub bone: Vec<(i32, i32)>,
    pub steps: usize,
}
