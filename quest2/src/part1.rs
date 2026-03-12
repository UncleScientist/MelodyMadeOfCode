use std::collections::HashSet;

use crate::DrawState;

#[derive(Default)]
pub struct Part1Solver {
    visited: HashSet<(i32, i32)>,
    cur_loc: (i32, i32),
    pub steps: usize,
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

    pub fn state(&self) -> DrawState<'_> {
        DrawState {
            bounding_box: (-50, -50, 50, 50),
            visited: self.visited.iter(),
            cur_loc: self.cur_loc,
            bone: vec![self.bone],
            steps: self.steps,
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
    solver.by_ref().count();
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
