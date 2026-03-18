use std::{convert::Infallible, fmt::Display, str::FromStr};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Socket {
    color: Color,
    shape: Shape,
}

impl FromStr for Socket {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (color, shape) = line.split_once(' ').unwrap();
        Ok(Self {
            color: color.parse().unwrap(),
            shape: shape.parse().unwrap(),
        })
    }
}

impl Socket {
    pub fn matches(&self, other: &Self) -> bool {
        self.color == other.color || self.shape == other.shape
    }

    pub fn is_weak(&self, other: &Socket) -> bool {
        assert!(self.matches(other));
        self.color != other.color || self.shape != other.shape
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.color, self.shape)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
enum Shape {
    #[default]
    Triangle,
    Square,
    Diamond,
    Pentagon,
    Star,
    Hexagon,
    Octagon,
    Circle,
}

impl FromStr for Shape {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(match line {
            "TRIANGLE" => Self::Triangle,
            "SQUARE" => Self::Square,
            "DIAMOND" => Self::Diamond,
            "PENTAGON" => Self::Pentagon,
            "STAR" => Self::Star,
            "HEXAGON" => Self::Hexagon,
            "OCTAGON" => Self::Octagon,
            "CIRCLE" => Self::Circle,
            _ => panic!("Unknown shape {line}"),
        })
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Triangle => "TRIANGLE",
                Self::Square => "SQUARE",
                Self::Diamond => "DIAMOND",
                Self::Pentagon => "PENTAGON",
                Self::Star => "STAR",
                Self::Hexagon => "HEXAGON",
                Self::Octagon => "OCTAGON",
                Self::Circle => "CIRCLE",
            }
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
enum Color {
    #[default]
    Blue,
    White,
    Green,
    Red,
    Black,
    Cyan,
    Magenta,
    Yellow,
}

impl FromStr for Color {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(match line {
            "BLUE" => Self::Blue,
            "WHITE" => Self::White,
            "GREEN" => Self::Green,
            "RED" => Self::Red,
            "BLACK" => Self::Black,
            "CYAN" => Self::Cyan,
            "MAGENTA" => Self::Magenta,
            "YELLOW" => Self::Yellow,
            _ => panic!("Unknown color {line}"),
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blue => "BLUE",
                Self::White => "WHITE",
                Self::Green => "GREEN",
                Self::Red => "RED",
                Self::Black => "BLACK",
                Self::Cyan => "CYAN",
                Self::Magenta => "MAGENTA",
                Self::Yellow => "YELLOW",
            }
        )
    }
}
