use roll::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: roll [DICE...]");
        println!("e.g. roll 3d6");
        println!();
        println!("Available Dice:");
        println!("\tdN (where N is a number) - N sided die");
        println!("\tdF - Fude dice: +, +, -, -, _, _");
        println!("\td% - A number between 1 and 100");
        std::process::exit(1);
    }
    let mut dice = DiceBag::new();
    let mut total: i32 = 0;
    for s in args[1..].iter() {
        match parse_rolls(s.trim()) {
            Ok(rolls) => {
                for roll in rolls.iter() {
                    for result in dice.roll(roll).iter() {
                        total += result;
                        println!("{}", result);
                    }
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
    println!("Total: {}", total);
}
