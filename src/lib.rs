use rand::prelude::*;

struct DiceBag {
    rng: ThreadRng,
}

impl DiceBag {
    fn new() -> DiceBag {
        DiceBag {
            rng: rand::thread_rng(),
        }
    }

    fn roll(&mut self, roll: &Roll) -> Vec<u32> {
        roll.roll(&mut self.rng)
    }
}

struct Roll {
    dice_count: u32,
    sides: u32,
}

impl Roll {
    fn roll<T : Rng>(&self, rng: &mut T) -> Vec<u32> {
        let mut results = Vec::new();
        for i in 0..self.dice_count {
            results.push(rng.gen_range(1, self.sides + 1));
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        let mut bag = DiceBag::new();
        let results = bag.roll(&Roll { dice_count: 2, sides: 8 });
        for i in 0..2 {
            let n = results[i];
            assert!(n >= 1 && n <= 8);
        }
    }
}
