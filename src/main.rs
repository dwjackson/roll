use roll::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
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
