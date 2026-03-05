use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e3_q01_p1.txt").expect("file");
    let scales: Vec<Scale> = data.lines().map(|line| line.parse().unwrap()).collect();
    println!(
        "part 1 = {}",
        scales
            .iter()
            .filter(|scale| scale.red < scale.green && scale.blue < scale.green)
            .map(|scale| scale.id)
            .sum::<usize>()
    );

    let data = std::fs::read_to_string("input/everybody_codes_e3_q01_p2.txt").expect("file");
    let scales: Vec<Scale> = data.lines().map(|line| line.parse().unwrap()).collect();
    let max_shine = scales.iter().map(|scale| scale.shine).max().unwrap();
    let darkest = scales
        .iter()
        .filter(|scale| scale.shine == max_shine)
        .map(|scale| (scale.red + scale.green + scale.blue, scale))
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap();
    println!("part 2 = {}", darkest.1.id);
}

#[derive(Default, Debug)]
struct Scale {
    id: usize,
    red: u8,
    green: u8,
    blue: u8,
    shine: u8,
}

impl FromStr for Scale {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, color) = line.split_once(':').unwrap();
        let rgb = color.split(' ').collect::<Vec<_>>();
        Ok(Scale {
            id: id.parse().unwrap(),
            red: rgb[0].to_binary(),
            green: rgb[1].to_binary(),
            blue: rgb[2].to_binary(),
            shine: if rgb.len() > 3 { rgb[3].to_binary() } else { 0 },
        })
    }
}

trait ToBinary {
    fn to_binary(&self) -> u8;
}

impl ToBinary for String {
    fn to_binary(&self) -> u8 {
        to_binary_impl(self)
    }
}

impl ToBinary for &str {
    fn to_binary(&self) -> u8 {
        to_binary_impl(self)
    }
}

fn to_binary_impl<S: AsRef<str>>(s: S) -> u8 {
    let mut result = 0;
    for ch in s.as_ref().chars() {
        result = (result << 1) | (ch.is_uppercase() as u8);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_binary() {
        let result = "rrrrrr".to_binary();
        assert_eq!(0, result);

        let result = "rrrrrR".to_binary();
        assert_eq!(1, result);

        let result = "rRrrrR".to_binary();
        assert_eq!(17, result);
    }

    #[test]
    fn test_create_scale() {
        let scale: Scale = "2456:rrrrrr ggGgGG bbbbBB".parse().unwrap();
        assert_eq!(scale.id, 2456);
        assert_eq!(scale.red, 0);
        assert_eq!(scale.green, 11);
        assert_eq!(scale.blue, 3);
    }

    #[test]
    fn test_shiny_scale() {
        let scale: Scale = "2456:rrrrrr ggGgGG bbbbBB sSsSsS".parse().unwrap();
        assert_eq!(scale.id, 2456);
        assert_eq!(scale.red, 0);
        assert_eq!(scale.green, 11);
        assert_eq!(scale.blue, 3);
        assert_eq!(scale.shine, 21);
    }
}
