use std::{convert::Infallible, fmt::Display, str::FromStr};

use crate::socket::Socket;

#[derive(Debug, Default, Clone)]
pub struct Node {
    id: usize,
    plug: Socket,

    left_socket: Socket,
    left: Option<Box<Node>>,

    right_socket: Socket,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn strong_insert(&mut self, new_node: Node) -> bool {
        if let Some(ref mut left) = self.left {
            // TODO: don't clone
            if left.strong_insert(new_node.clone()) {
                return true;
            }
        } else if new_node.plug == self.left_socket {
            self.left = Some(Box::new(new_node));
            return true;
        }

        if let Some(ref mut right) = self.right {
            if right.strong_insert(new_node) {
                return true;
            }
        } else if new_node.plug == self.right_socket {
            self.right = Some(Box::new(new_node));
            return true;
        }
        false
    }

    pub fn weak_insert(&mut self, new_node: Node) -> bool {
        if let Some(ref mut left) = self.left {
            // TODO: don't clone
            if left.weak_insert(new_node.clone()) {
                return true;
            }
        } else if new_node.plug.matches(&self.left_socket) {
            self.left = Some(Box::new(new_node));
            return true;
        }

        if let Some(ref mut right) = self.right {
            if right.weak_insert(new_node) {
                return true;
            }
        } else if new_node.plug.matches(&self.right_socket) {
            self.right = Some(Box::new(new_node));
            return true;
        }
        false
    }

    pub fn rebonding_insert(&mut self, mut next: Node) -> Option<Node> {
        if let Some(ref mut left) = self.left {
            if left.plug.is_weak(&self.left_socket) && next.plug == self.left_socket {
                let old = left.clone();
                **left = next;
                next = *old;
            } else {
                next = left.rebonding_insert(next)?;
            }
        } else if next.plug.matches(&self.left_socket) {
            self.left = Some(Box::new(next));
            return None;
        }

        if let Some(ref mut right) = self.right {
            if right.plug.is_weak(&self.right_socket) && next.plug == self.right_socket {
                let old = right.clone();
                **right = next;
                next = *old;
            } else {
                next = right.rebonding_insert(next)?;
            }
        } else if next.plug.matches(&self.right_socket) {
            self.right = Some(Box::new(next));
            return None;
        }

        Some(next)
    }

    pub fn order(&self, ids: &mut Vec<usize>) {
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
            "{:indent$} {} plug={} left={} right={}",
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
            plug: plug.parse().unwrap(),
            left_socket: left_socket.parse().unwrap(),
            right_socket: right_socket.parse().unwrap(),
            ..Self::default()
        })
    }
}
