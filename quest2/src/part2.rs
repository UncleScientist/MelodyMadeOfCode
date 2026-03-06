use std::{collections::HashSet, fmt::Display};

pub fn run(bone: &(i32, i32)) -> usize {
    let mut wave = Wave::new(bone);
    let mut loc = (0i32, 0i32);
    let mut steps: usize = 0;

    while !wave.surrounds_bone() {
        steps += 1;
        loc = loop {
            let next_loc = wave.next(&loc);
            if wave.visit(next_loc) {
                break next_loc;
            }
        };

        wave.check_for_gap(&loc);
    }

    steps
}

#[derive(Default)]
struct Wave {
    visited: HashSet<(i32, i32)>,
    bone: (i32, i32),
    dir: usize,
    top: i32,
    bot: i32,
    left: i32,
    right: i32,
}

impl Display for Wave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Vert: {}..{}, Horiz: {}..{}",
            self.top, self.bot, self.left, self.right
        )?;
        for row in self.top..=self.bot {
            for col in self.left..=self.right {
                if (row, col) == self.bone {
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

impl Wave {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn new(bone: &(i32, i32)) -> Self {
        Wave {
            visited: HashSet::from([(0, 0)]),
            bone: *bone,
            top: 0.min(bone.0),
            bot: 0.max(bone.0),
            left: 0.min(bone.1),
            right: 0.max(bone.1),
            dir: 0,
        }
    }

    fn next(&mut self, loc: &(i32, i32)) -> (i32, i32) {
        let curdir = self.dir;
        self.dir = (self.dir + 1) % 4;
        (loc.0 + Self::DIRS[curdir].0, loc.1 + Self::DIRS[curdir].1)
    }

    fn surrounds_bone(&self) -> bool {
        for delta in &Self::DIRS {
            let pos = (delta.0 + self.bone.0, delta.1 + self.bone.1);
            if !self.visited.contains(&pos) {
                return false;
            }
        }
        true
    }

    fn visit(&mut self, next_loc: (i32, i32)) -> bool {
        self.top = self.top.min(next_loc.0);
        self.bot = self.bot.max(next_loc.0);

        self.left = self.left.min(next_loc.1);
        self.right = self.right.max(next_loc.1);

        if next_loc == self.bone {
            return false;
        }
        self.visited.insert(next_loc)
    }

    fn check_for_gap(&mut self, loc: &(i32, i32)) {
        for dir in &Self::DIRS {
            let pos = if dir.0 == 0 {
                [
                    (loc.0 - 1, loc.1 + dir.1),
                    (loc.0, loc.1 + dir.1),
                    (loc.0 + 1, loc.1 + dir.1),
                ]
            } else {
                [
                    (loc.0 + dir.0, loc.1 - 1),
                    (loc.0 + dir.0, loc.1),
                    (loc.0 + dir.0, loc.1 + 1),
                ]
            };
            if (self.visited.contains(&pos[0]) || pos[0] == self.bone)
                && (self.visited.contains(&pos[2]) || pos[2] == self.bone)
                && (!self.visited.contains(&pos[1]) && pos[1] != self.bone)
            {
                self.flood_fill(&pos[1]);
            }
        }
    }

    fn flood_fill(&mut self, pos: &(i32, i32)) {
        assert!(!self.visited.contains(pos));
        'next_dir: for dir in Self::DIRS {
            let mut check = *pos;
            loop {
                check = (check.0 + dir.0, check.1 + dir.1);
                if self.visited.contains(&check) || self.bone == check {
                    continue 'next_dir;
                }
                if check.0 < self.top
                    || check.0 > self.bot
                    || check.1 < self.left
                    || check.1 > self.right
                {
                    return;
                }
            }
        }

        let mut queue = Vec::from([*pos]);

        while let Some(loc) = queue.pop() {
            if self.visited.insert(loc) {
                for dir in &Self::DIRS {
                    let next_loc = (dir.0 + loc.0, dir.1 + loc.1);
                    if !self.visited.contains(&next_loc) && self.bone != next_loc {
                        queue.push(next_loc);
                    }
                }
            }
        }
    }
}
