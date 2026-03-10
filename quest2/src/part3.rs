use std::{collections::HashSet, fmt::Display};

pub fn run(bones: Vec<(i32, i32)>) -> usize {
    let solver = Solver::new(bones);

    println!("{solver}");

    while !solver.finished() {
        solver.step();
    }

    solver.steps
}

struct Solver {
    _curloc: (i32, i32),
    steps: usize,
    visited: HashSet<(i32, i32)>,
    structure: HashSet<(i32, i32)>,
    remaining: HashSet<(i32, i32)>,
    _curdir: usize,
    bounding_box: (i32, i32, i32, i32), // top, bottom, left, right
}

impl Solver {
    fn new(bones: Vec<(i32, i32)>) -> Self {
        let structure: HashSet<(i32, i32)> = bones.into_iter().collect();
        let mut remaining = HashSet::new();
        let mut bounding_box = (0, 0, 0, 0);

        for point in &structure {
            for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let pt = (point.0 + delta.0, point.1 + delta.1);
                if !structure.contains(&pt) {
                    remaining.insert(pt);
                    bounding_box = (
                        bounding_box.0.min(pt.0),
                        bounding_box.1.max(pt.0),
                        bounding_box.2.min(pt.1),
                        bounding_box.3.max(pt.1),
                    );
                }
            }
        }

        Self {
            _curloc: (0, 0),
            steps: 0,
            visited: HashSet::from([(0, 0)]),
            remaining,
            structure,
            _curdir: 0,
            bounding_box,
        }
    }

    fn finished(&self) -> bool {
        todo!()
    }

    fn step(&self) {
        todo!()
    }

    const _DIRS: [(i32, i32); 12] = [
        (-1, 0),
        (-1, 0),
        (-1, 0),
        (0, 1),
        (0, 1),
        (0, 1),
        (1, 0),
        (1, 0),
        (1, 0),
        (0, -1),
        (0, -1),
        (0, -1),
    ];
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-- Step {} --", self.steps)?;
        for row in self.bounding_box.0..=self.bounding_box.1 {
            for col in self.bounding_box.2..=self.bounding_box.3 {
                if self.remaining.contains(&(row, col)) {
                    write!(f, "x")?;
                } else if self.structure.contains(&(row, col)) {
                    write!(f, "#")?;
                } else if self.visited.contains(&(row, col)) {
                    write!(f, "+")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
