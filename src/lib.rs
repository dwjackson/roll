use rand::prelude::*;

fn roll<T : Rng>(sides: u32, rng: &mut T) -> u32 {
    rng.gen_range(1, sides + 1)
}

struct DiceBag {
    rng: ThreadRng,
}

impl DiceBag {
    fn new() -> DiceBag {
        DiceBag {
            rng: rand::thread_rng(),
        }
    }

    fn roll(&mut self, sides: u32) -> u32 {
        roll(sides, &mut self.rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_6_sided_die() {
        let mut rng = rand::thread_rng();
        let n = roll(6, &mut rng);
        assert!(n >= 1 && n <= 6);
    }

    #[test]
    fn test_dice_bag() {
        let mut bag = DiceBag::new();
        let n = bag.roll(8);
        assert!(n >= 1 && n <= 8);
    }
}
