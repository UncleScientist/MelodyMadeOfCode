use std::collections::HashSet;

pub fn run(bone: &(i32, i32)) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut loc = (0i32, 0i32);
    let mut steps = 0;
    let mut dir = 0;

    visited.insert(loc);
    while loc != *bone {
        // print_grid(steps, &visited);
        steps += 1;
        loc = loop {
            let next_loc = (loc.0 + DIRS[dir].0, loc.1 + DIRS[dir].1);
            dir = (dir + 1) % 4;
            if visited.insert(next_loc) {
                // println!("visited {next_loc:?}");
                break next_loc;
            }
        };
    }
    // print_grid(steps, &visited);
    steps
}

fn _print_grid(steps: usize, visited: &HashSet<(i32, i32)>) {
    println!("-- Step {steps} --");
    for row in -10..10 {
        for col in -10..10 {
            if visited.contains(&(row, col)) {
                print!("+");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
