use rand::prelude::*;
use regex::Regex;
use std::str::FromStr;

pub struct DiceBag {
    rng: ThreadRng,
}

impl DiceBag {
    pub fn new() -> DiceBag {
        DiceBag {
            rng: rand::thread_rng(),
        }
    }

    pub fn roll(&mut self, roll: &Roll) -> Vec<i32> {
        roll.roll(&mut self.rng)
    }
}

impl Default for DiceBag {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Roll {
    dice_count: u32,
    sides: u32,
    values: Vec<i32>,
}

impl Roll {
    pub fn roll<T: Rng>(&self, rng: &mut T) -> Vec<i32> {
        let mut results = Vec::new();
        for _ in 0..self.dice_count {
            let index = rng.gen_range(0, self.sides) as usize;
            let result = self.values[index];
            results.push(result);
        }
        results
    }
}

impl FromStr for Roll {
    type Err = ParseRollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+)d(\d+)").unwrap();
        match re.captures(s) {
            Some(c) => {
                let dice_count = match c[1].parse() {
                    Ok(n) => n,
                    Err(_) => return Err(ParseRollError::InvalidDiceCount),
                };
                let sides = match c[2].parse() {
                    Ok(n) => n,
                    Err(_) => return Err(ParseRollError::InvalidSides),
                };
                if sides < 2 || sides == 3 {
                    return Err(ParseRollError::ImpossibleDie(sides));
                }
                let roll = Roll {
                    dice_count,
                    sides,
                    values: normal_die(sides),
                };
                Ok(roll)
            }
            None => Err(ParseRollError::InvalidRoll),
        }
    }
}

fn normal_die(sides: u32) -> Vec<i32> {
    let mut values = Vec::new();
    for i in 1..(sides + 1) {
        values.push(i as i32);
    }
    values
}

pub fn parse_rolls(s: &str) -> Result<Vec<Roll>, ParseRollError> {
    let mut rolls = Vec::new();
    for roll_str in s.split_whitespace() {
        let roll = roll_str.parse()?;
        rolls.push(roll);
    }
    Ok(rolls)
}

#[derive(Debug)]
pub enum ParseRollError {
    InvalidRoll,
    InvalidDiceCount,
    InvalidSides,
    ImpossibleDie(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        let mut bag = DiceBag::new();
        let results = bag.roll(&Roll {
            dice_count: 2,
            sides: 8,
            values: vec![1, 2, 3, 4, 5, 6, 7, 8],
        });
        for i in 0..2 {
            let n = results[i];
            assert!(n >= 1 && n <= 8);
        }
    }

    #[test]
    fn test_parse_roll() {
        let s = "3d6";
        let roll: Roll = s.parse().expect("Bad parse");
        assert_eq!(roll.dice_count, 3);
        assert_eq!(roll.sides, 6);
        let mut rng = rand::thread_rng();
        let results = roll.roll(&mut rng);
        assert_eq!(results.len(), 3);
        // TODO
    }

    #[test]
    fn test_parse_multiple_roles() {
        let s = "3d6 2d8 1d20";
        let rolls = parse_rolls(s).expect("Bad parse");
        assert_eq!(rolls.len(), 3);
    }

    #[test]
    fn test_impossible_dice_shape_1_side() {
        let s = "2d1";
        match s.parse::<Roll>() {
            Ok(_) => panic!("Impossible shape"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_impossible_dice_shape_3_sides() {
        let s = "3d3";
        match s.parse::<Roll>() {
            Ok(_) => panic!("Impossible shape"),
            Err(_) => (),
        }
    }

    #[test]
    #[ignore]
    fn test_parse_custom() {
        let s = "1d{1,1,0,0,-1,-1}";
        let roll: Roll = s.parse().expect("Bad parse");
        assert_eq!(roll.sides, 6);
        assert_eq!(roll.values, vec![1, 1, 0, 0, -1, -1]);
    }
}
