use std::{collections::HashSet, fmt::Display};

use crate::DrawState;

pub fn run(bones: Vec<(i32, i32)>) -> usize {
    let mut solver = Part3Solver::new(bones);

    while solver.next().is_some() && solver.steps < 16000 {
        // do nothing
    }

    solver.steps
}

pub struct Part3Solver {
    curloc: (i32, i32),
    steps: usize,
    visited: HashSet<(i32, i32)>,
    structure: HashSet<(i32, i32)>,
    remaining: HashSet<(i32, i32)>,
    curdir: usize,
    bounding_box: (i32, i32, i32, i32), // top, bottom, left, right
}

impl Iterator for Part3Solver {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished() {
            None
        } else {
            self.step();
            Some(())
        }
    }
}

impl Part3Solver {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    const DIR3STEP: [(i32, i32); 12] = [
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

    pub fn new(bones: Vec<(i32, i32)>) -> Self {
        let structure: HashSet<(i32, i32)> = bones.into_iter().collect();
        let bounding_box = structure.iter().fold((0, 0, 0, 0), |bb, pt| {
            (
                bb.0.min(pt.0 - 1),
                bb.1.max(pt.0 + 1),
                bb.2.min(pt.1 - 1),
                bb.3.max(pt.1 + 1),
            )
        });

        let remaining = Self::find_enclosure(&structure, &bounding_box);

        Self {
            curloc: (0, 0),
            steps: 0,
            visited: HashSet::from([(0, 0)]),
            remaining,
            structure,
            curdir: 0,
            bounding_box,
        }
    }

    pub fn state(&self) -> DrawState<'_> {
        DrawState {
            bounding_box: self.bounding_box,
            visited: self.visited.iter(),
            cur_loc: self.curloc,
            bone: self.structure.iter().copied().collect(),
            steps: self.steps,
        }
    }

    fn finished(&self) -> bool {
        self.remaining.is_empty()
    }

    // ++++++
    // +    +
    // +   s+      #
    // +####
    fn step(&mut self) {
        self.steps += 1;

        let mut failsafe = 0;
        let next_loc = loop {
            let next_loc = (
                self.curloc.0 + Self::DIR3STEP[self.curdir].0,
                self.curloc.1 + Self::DIR3STEP[self.curdir].1,
            );
            self.curdir = (self.curdir + 1) % Self::DIR3STEP.len();
            if !self.visited.contains(&next_loc) && !self.structure.contains(&next_loc) {
                break next_loc;
            }
            failsafe += 1;
            if failsafe > Self::DIR3STEP.len() {
                panic!("Looped {failsafe} times in step() function");
            }
        };
        self.curloc = next_loc;
        self.bounding_box = (
            self.bounding_box.0.min(next_loc.0),
            self.bounding_box.1.max(next_loc.0),
            self.bounding_box.2.min(next_loc.1),
            self.bounding_box.3.max(next_loc.1),
        );
        self.remaining.remove(&next_loc);
        self.visited.insert(next_loc);
        self.flood_fill_from(&next_loc);
    }

    fn flood_fill_from(&mut self, next_loc: &(i32, i32)) {
        for candidate in self.find_candidates(next_loc) {
            if let Some(visited) = self.try_flood(candidate) {
                self.visited.extend(&visited);
                self.remaining.retain(|p| !visited.contains(p));
            }
        }
    }

    fn find_candidates(&self, loc: &(i32, i32)) -> Vec<(i32, i32)> {
        Self::DIRS
            .iter()
            .map(|dir| (loc.0 + dir.0, loc.1 + dir.1))
            .filter(|pt| !self.visited.contains(pt) && !self.structure.contains(pt))
            .collect()
    }

    fn try_flood(&self, start: (i32, i32)) -> Option<HashSet<(i32, i32)>> {
        let mut queue = vec![start];
        let mut visited = HashSet::new();

        while let Some(pos) = queue.pop() {
            if visited.insert(pos) {
                for delta in &Self::DIRS {
                    let pt = (pos.0 + delta.0, pos.1 + delta.1);
                    if pt.0 < self.bounding_box.0
                        || pt.0 > self.bounding_box.1
                        || pt.1 < self.bounding_box.2
                        || pt.1 > self.bounding_box.3
                    {
                        return None;
                    }
                    if !visited.contains(&pt)
                        && !self.visited.contains(&pt)
                        && !self.structure.contains(&pt)
                    {
                        queue.push(pt);
                    }
                }
            }
        }

        Some(visited)
    }

    fn find_enclosure(
        structure: &HashSet<(i32, i32)>,
        bounding_box: &(i32, i32, i32, i32),
    ) -> HashSet<(i32, i32)> {
        let mut enclosure = HashSet::new();

        let mut queue = vec![(bounding_box.0, bounding_box.2)];
        let mut visited = HashSet::new();
        while let Some(pos) = queue.pop() {
            if visited.insert(pos) {
                for delta in &Self::DIRS {
                    let pt = (delta.0 + pos.0, delta.1 + pos.1);
                    if pt.0 < bounding_box.0
                        || pt.0 > bounding_box.1
                        || pt.1 < bounding_box.2
                        || pt.1 > bounding_box.3
                    {
                        continue;
                    }
                    if structure.contains(&pt) {
                        enclosure.insert(pos);
                        continue;
                    }
                    if !visited.contains(&pt) {
                        queue.push(pt);
                    }
                }
            }
        }

        // Remove the starting location in case we hit that already
        enclosure.remove(&(0, 0));

        enclosure
    }
}

impl Display for Part3Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-- Step {} --", self.steps)?;
        for row in self.bounding_box.0..=self.bounding_box.1 {
            for col in self.bounding_box.2..=self.bounding_box.3 {
                if self.curloc == (row, col) {
                    write!(f, "@")?;
                } else if self.remaining.contains(&(row, col)) {
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
