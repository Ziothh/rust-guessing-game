use std::{io, cmp::Ordering};

use rand::{thread_rng, rngs::ThreadRng, Rng};

fn main() {
    let mut rng = thread_rng();

    println!("Guess the number");

    
    loop {
        let mut guess = String::new();

        println!("Please input a number (0..10): ");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read your input");
     
        if (guess == "stop") {break;}

        let number_to_guess = rng.gen_range(0..10);

        println!("You guessed {guess}");

        match guess.trim().parse::<i32>() {
            Ok(n) => {
                match n.cmp(&number_to_guess) {
                    Ordering::Less => println!("Too small! Number was {number_to_guess}"),
                    Ordering::Greater => println!("Too big! Number was {number_to_guess}"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    },
                }
            },
            Err(_e) => {
                println!("\"{guess}\" is not a number.")
            },
          }

       
    }



}
