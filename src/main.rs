use rand::prelude::*;
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
        println!("\tdF - Fudge dice: +, +, -, -, _, _");
        println!("\td% - A number between 1 and 100");
        std::process::exit(1);
    }
    let mut dice = DiceBag::new(thread_rng());
    for s in args[1..].iter() {
        match parse_rolls(s.trim()) {
            Ok(rolls) => {
                let results = dice.roll_all(&rolls);
                if results.values.len() > 1 {
                    let description = rolls
                        .iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    let values = results
                        .values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    println!(
                        "{description}: {values} (Total: {}, Highest: {}, Lowest: {})",
                        results.total, results.highest, results.lowest
                    );
                } else {
                    for value in results.values.iter() {
                        println!("{}", value);
                    }
                }
            }
            Err(e) => {
                println!("Error: {e:?}");
                std::process::exit(1);
            }
        }
    }
}
