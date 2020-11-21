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

    pub fn roll(&mut self, roll: &Roll) -> Vec<u32> {
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
}

impl Roll {
    pub fn roll<T: Rng>(&self, rng: &mut T) -> Vec<u32> {
        let mut results = Vec::new();
        for _ in 0..self.dice_count {
            results.push(rng.gen_range(1, self.sides + 1));
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
                Ok(Roll { dice_count, sides })
            }
            None => Err(ParseRollError::InvalidRoll),
        }
    }
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
    }

    #[test]
    fn test_parse_multiple_roles() {
        let s = "3d6 2d8 1d20";
        let rolls = parse_rolls(s).expect("Bad parse");
        assert_eq!(rolls.len(), 3);
    }
}
