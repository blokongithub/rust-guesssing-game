use std::cmp::max;
use std::io::{stdin,stdout,Write};
use rand::Rng; //test

fn main() {
    println!("Hello, world!");
    let minimum: u8 = 1;
    let maximum: u8 = 100;
    let mut awnser: u8;
    awnser = randomnumber(minimum, maximum);
    let mut gameplaying: bool = true;
    let mut wannaplayagain: bool = true;
    while gameplaying == true {
        gameplaying = game(minimum, maximum, awnser)
    }
}

pub fn game(minimum: u8, maximum: u8, awnser: u8) -> bool{
    let mut guessstr = String::new();
    println!("please enter a value between {} and {}", minimum, maximum);
    let b1 = std::io::stdin().read_line(&mut guessstr).unwrap();
    match guessstr.trim().parse::<u8>() {
        Ok(guess) if guess >= minimum && guess <= maximum => {
            if guess == awnser {
                println!("You win, you guess correct");
                return false;
            } else if guess > awnser {
                println!("Too high!");
            } else {
                println!("Too low");
            }
            return true;
        }
        Ok(_) => {
            println!("The number is out of range. Please enter a value between {} and {}", minimum, maximum);
            return true;
        }
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return true;
        }
    }
}

pub fn randomnumber(minimum: u8, maximum: u8) -> u8 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(minimum..maximum);
}
