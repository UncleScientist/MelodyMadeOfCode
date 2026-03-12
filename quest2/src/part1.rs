use std::collections::HashSet;

#[derive(Default)]
pub struct Part1Solver {
    visited: HashSet<(i32, i32)>,
    cur_loc: (i32, i32),
    steps: usize,
    cur_dir: usize,
    bone: (i32, i32),
}

impl Part1Solver {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    pub fn new(bone: (i32, i32)) -> Self {
        let visited = HashSet::<(i32, i32)>::from([(0, 0)]);
        Self {
            visited,
            bone,
            ..Self::default()
        }
    }
}

impl Iterator for Part1Solver {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.steps += 1;
        self.cur_loc = loop {
            let next_loc = (
                self.cur_loc.0 + Self::DIRS[self.cur_dir].0,
                self.cur_loc.1 + Self::DIRS[self.cur_dir].1,
            );
            self.cur_dir = (self.cur_dir + 1) % 4;
            if self.visited.insert(next_loc) {
                // println!("visited {next_loc:?}");
                break next_loc;
            }
        };
        if self.cur_loc == self.bone {
            None
        } else {
            Some(())
        }
    }
}

pub fn run(bone: (i32, i32)) -> usize {
    let mut solver = Part1Solver::new(bone);
    for () in solver.by_ref() {
        // do nothing
    }
    solver.steps
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
