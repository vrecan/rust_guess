use std::io;
use rand::Rng;


fn main() {
    let mut rng = rand::thread_rng();
    println!("Guess the number!");
    let mut guess;
    while !quit {
        let mut quit = false;
        let secret = rng.gen_range(0, 20);
        guess = String::new();
        println!("Please input your guess between 0-20 or Q to quit.");
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        match guess.trim().parse::<i32>() {
        Ok(n) => check_guess(n, secret),
        Err(_) => quit = should_quit(guess)
        }
    }

    fn check_guess(guess: i32, secret: i32) {
        if guess == secret {
            println!("Congrats you guessed it correctly!")
        } else {
            println!("Wrong, the answer was {}", secret)
        }
    }

    fn should_quit(guess: String) -> bool {
        let res = match guess.trim().to_lowercase().as_str() {
            "q" => true,
            _ => false
        };
        return res;
    }
}