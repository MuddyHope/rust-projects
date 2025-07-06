use std::io;
use std::cmp::Ordering;
use rand::{Rng};

enum Difficulty {
    E,
    M,
    H
}

fn main() {
    println!("Hello Welcome to the guessing number game");


    let mut rng = rand::thread_rng();

    println!("What should be the difficulty ??\n Easy-> E\n Medium -> M\n Hard -> H");

    let mut difficulty_input = String::new();

    io::stdin().read_line(&mut difficulty_input).expect("Failed to read line");
    let difficulty_input = difficulty_input.trim();

    let difficulty = match difficulty_input {
        "E" | "e" => Difficulty::E,
        "M" | "m" => Difficulty::M,
        "H" | "h" => Difficulty::H,
        _ => {
            println!("Invalid difficulty, defaulting to Easy.");
            Difficulty::E
        }
    };

    let random_number: u32 = match difficulty {
        Difficulty::E => rng.gen_range(1..=101),
        Difficulty::M => rng.gen_range(1..=501),
        Difficulty::H => rng.gen_range(1..=1001)
    };


    println!("Now try taking a guess for a number.");



    loop {
        let mut guessed_random: String = String::new();

        io::stdin().read_line(&mut guessed_random).expect("Failed to read line");

        println!("The number you guessed was : {}", guessed_random);
    
        let guessed_random: u32 = match guessed_random.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };


        match guessed_random.cmp(&random_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
        };
    }


    
}
