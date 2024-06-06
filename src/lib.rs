use rand::prelude::*;
use regex::Regex;
use std::fmt;
use std::str::FromStr;

pub struct DiceBag<T>
where
    T: Rng,
{
    rng: T,
}

impl<T> DiceBag<T>
where
    T: Rng,
{
    pub fn new(rng: T) -> DiceBag<T> {
        DiceBag { rng }
    }

    pub fn roll(&mut self, roll: &Roll) -> Vec<i32> {
        roll.roll(&mut self.rng)
    }

    pub fn roll_all(&mut self, rolls: &[Roll]) -> RollResults {
        let mut total = 0;
        let mut lowest = std::i32::MAX;
        let mut highest = 0;
        let mut values: Vec<i32> = Vec::new();
        for roll in rolls.iter() {
            let results = self.roll(roll);
            for &result in results.iter() {
                if result < lowest {
                    lowest = result;
                }
                if result > highest {
                    highest = result;
                }
                total += result;
                values.push(result);
            }
        }
        RollResults {
            total,
            lowest,
            highest,
            values,
        }
    }
}

#[derive(Debug)]
pub struct Roll {
    dice_count: u32,
    sides: u32,
    values: Vec<i32>,
}

impl Roll {
    pub fn roll<T: Rng>(&self, rng: &mut T) -> Vec<i32> {
        let mut results = Vec::new();
        for _ in 0..self.dice_count {
            let index = rng.gen_range(0..self.sides) as usize;
            let result = self.values[index];
            results.push(result);
        }
        results
    }
}

impl FromStr for Roll {
    type Err = ParseRollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+)?d(.+)").unwrap();
        match re.captures(s) {
            Some(c) => {
                let dice_count = c.get(1).map_or(1, |m| m.as_str().parse().unwrap_or(0));
                if dice_count == 0 {
                    return Err(ParseRollError::InvalidDiceCount);
                }
                let (sides, values) = parse_sides(&c[2])?;
                if sides < 2 || sides == 3 {
                    return Err(ParseRollError::ImpossibleDie(sides));
                }
                let roll = Roll {
                    dice_count,
                    sides,
                    values,
                };
                Ok(roll)
            }
            None => Err(ParseRollError::InvalidRoll),
        }
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.dice_count, self.sides)
    }
}

fn parse_sides(s: &str) -> Result<(u32, Vec<i32>), ParseRollError> {
    let mut chars = s.chars();
    let c = chars.next().unwrap();
    if c == '{' {
        parse_custom_die(s)
    } else if c == 'F' || c == 'f' {
        Ok((6, vec![1, 1, 0, 0, -1, -1]))
    } else if c == '%' {
        Ok((100, (1..101).collect()))
    } else {
        match s.parse::<i32>() {
            Ok(n) => Ok((n as u32, normal_die(n as u32))),
            Err(_) => Err(ParseRollError::InvalidSides),
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

fn parse_custom_die(s: &str) -> Result<(u32, Vec<i32>), ParseRollError> {
    let mut chars = s.chars();
    let c = chars.next().unwrap();
    if c != '{' {
        return Err(ParseRollError::CustomDieSyntaxError);
    }
    let values = parse_values(&mut chars)?;
    Ok((values.len() as u32, values))
}

fn parse_values(chars: &mut std::str::Chars) -> Result<Vec<i32>, ParseRollError> {
    let mut values = Vec::new();
    while let Some(value) = parse_value(chars)? {
        values.push(value);
    }
    Ok(values)
}

fn parse_value(chars: &mut std::str::Chars) -> Result<Option<i32>, ParseRollError> {
    let c = skip_whitespace(chars);
    if c.is_none() {
        return Ok(None);
    }
    let c = c.unwrap();
    let is_negative = c == '-';
    let mut digits = String::new();
    if !is_negative {
        if c.is_ascii_digit() {
            digits.push(c);
        } else {
            return Err(ParseRollError::InvalidDigit(c));
        }
    }
    for c in chars {
        if c.is_ascii_digit() {
            digits.push(c);
        } else if c == ',' || c == '}' {
            break;
        } else if c.is_whitespace() {
            // Skip trailing whitespace
        } else {
            return Err(ParseRollError::InvalidDieValue);
        }
    }
    let n = digits.parse::<i32>().unwrap();
    if is_negative {
        Ok(Some(-n))
    } else {
        Ok(Some(n))
    }
}

fn skip_whitespace(chars: &mut std::str::Chars) -> Option<char> {
    chars.find(|&c| !c.is_whitespace())
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
    CustomDieSyntaxError,
    InvalidDigit(char),
    InvalidDieValue,
}

pub struct RollResults {
    pub total: i32,
    pub lowest: i32,
    pub highest: i32,
    pub values: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        let mut bag = DiceBag::new(thread_rng());
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
    fn test_parse_custom() {
        let s = "1d{1,1,0,0,-1,-1}";
        let roll: Roll = s.parse().expect("Bad parse");
        assert_eq!(roll.sides, 6);
        assert_eq!(roll.values, vec![1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn test_parse_custom_with_preceding_whitespace() {
        let s = "1d{ 1,\t1, \t0,\t \t0,\t -1,   -1}";
        let roll: Roll = s.parse().expect("Bad parse");
        assert_eq!(roll.sides, 6);
        assert_eq!(roll.values, vec![1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn test_parse_custom_with_preceding_and_trailing_whitespace() {
        let s = "1d{ 1 ,\t1\t, \t0 \t,\t \t0\t \t,\t -1\t ,   -1   }";
        let roll: Roll = s.parse().expect("Bad parse");
        assert_eq!(roll.sides, 6);
        assert_eq!(roll.values, vec![1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn test_fudge_dice() {
        let s = "2dF";
        let roll: Roll = s.parse().expect("Bad parse");
        assert_eq!(roll.dice_count, 2);
        assert_eq!(roll.sides, 6);
        assert_eq!(roll.values, vec![1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn test_percentile_dice() {
        let s = "1d%";
        let roll: Roll = s.parse().expect("Bad parse");
        assert_eq!(roll.dice_count, 1);
        assert_eq!(roll.sides, 100);
        let values: Vec<i32> = (1..101).collect();
        assert_eq!(roll.values, values);
        assert_eq!(roll.values.len(), 100);
    }

    #[test]
    fn test_single_die_without_number_of_dice() {
        let s = "d20";
        let roll: Roll = s.parse().expect("Bad parse");
        assert_eq!(roll.dice_count, 1);
        assert_eq!(roll.sides, 20);
    }

    #[test]
    fn test_summary_of_rolls() {
        let input = "2d20 1d6";
        let rng = rand::rngs::mock::StepRng::new(3, 1);
        let mut dice = DiceBag::new(rng);
        let rolls = parse_rolls(input).expect("Bad parse");
        let result = dice.roll_all(&rolls);
        assert_eq!(result.total, 3);
        assert_eq!(result.lowest, 1);
        assert_eq!(result.highest, 1);
    }
}
