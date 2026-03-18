mod node;
mod socket;

use node::Node;

fn main() {
    println!("part 1 = {}", part1());
    println!("part 2 = {}", part2());
}

fn part1() -> usize {
    let file =
        std::fs::read_to_string("input/everybody_codes_e3_q03_p1.txt").expect("missing file");
    // std::fs::read_to_string("input/test-part-1.txt").expect("missing file");
    let mut nodes: Vec<Node> = file.lines().map(|line| line.parse().unwrap()).collect();

    let mut root = nodes.remove(0);
    while !nodes.is_empty() {
        let next = nodes.remove(0);
        root.strong_insert(next);
    }

    let mut order = Vec::new();
    root.order(&mut order);
    order
        .iter()
        .enumerate()
        .map(|(idx, value)| (idx + 1) * value)
        .sum::<usize>()
}

fn part2() -> usize {
    let file =
        std::fs::read_to_string("input/everybody_codes_e3_q03_p2.txt").expect("missing file");
    // std::fs::read_to_string("input/test-part-2.txt").expect("missing file");
    let mut nodes: Vec<Node> = file.lines().map(|line| line.parse().unwrap()).collect();

    let mut root = nodes.remove(0);
    while !nodes.is_empty() {
        let next = nodes.remove(0);
        root.weak_insert(next);
    }

    let mut order = Vec::new();
    root.order(&mut order);
    order
        .iter()
        .enumerate()
        .map(|(idx, value)| (idx + 1) * value)
        .sum::<usize>()
}
