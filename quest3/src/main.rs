use std::{convert::Infallible, fmt::Display, str::FromStr};

fn main() {
    let file =
        std::fs::read_to_string("input/everybody_codes_e3_q03_p1.txt").expect("missing file");
    // std::fs::read_to_string("input/test-part-1.txt").expect("missing file");
    let mut nodes: Vec<Node> = file.lines().map(|line| line.parse().unwrap()).collect();

    let mut root = nodes.remove(0);
    while !nodes.is_empty() {
        let next = nodes.remove(0);
        root.insert(next);
    }

    let mut order = Vec::new();
    root.order(&mut order);
    println!(
        "part 1 = {}",
        order
            .iter()
            .enumerate()
            .map(|(idx, value)| (idx + 1) * value)
            .sum::<usize>()
    );
}

#[derive(Debug, Default, Clone)]
struct Node {
    id: usize,
    plug: String,

    left_socket: String,
    left: Option<Box<Node>>,

    right_socket: String,
    right: Option<Box<Node>>,
}

impl Node {
    fn insert(&mut self, new_node: Node) -> bool {
        if let Some(ref mut left) = self.left {
            // TODO: don't clone
            if left.insert(new_node.clone()) {
                return true;
            }
        } else if new_node.plug == self.left_socket {
            self.left = Some(Box::new(new_node));
            return true;
        }

        if let Some(ref mut right) = self.right {
            if right.insert(new_node) {
                return true;
            }
        } else if new_node.plug == self.right_socket {
            self.right = Some(Box::new(new_node));
            return true;
        }
        false
    }

    fn order(&self, ids: &mut Vec<usize>) {
        if let Some(left) = &self.left {
            left.order(ids);
        }
        ids.push(self.id);
        if let Some(right) = &self.right {
            right.order(ids);
        }
    }

    fn display(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(left) = &self.left {
            left.display(indent + 1, f)?;
        }
        writeln!(
            f,
            "{:indent$} {} {} {} {}",
            "", self.id, self.plug, self.left_socket, self.right_socket
        )?;
        if let Some(right) = &self.right {
            right.display(indent + 1, f)?;
        }
        Ok(())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, f)
    }
}

impl FromStr for Node {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line.split(", ").collect::<Vec<_>>();
        let (_, id) = words[0].split_once('=').unwrap();
        let (_, plug) = words[1].split_once('=').unwrap();
        let (_, left_socket) = words[2].split_once('=').unwrap();
        let (_, right_socket) = words[3].split_once('=').unwrap();
        Ok(Self {
            id: id.parse().unwrap(),
            plug: plug.into(),
            left_socket: left_socket.into(),
            right_socket: right_socket.into(),
            ..Self::default()
        })
    }
}
