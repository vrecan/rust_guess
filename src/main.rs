use std::io;

fn main() {
    println!("Guess the number!");
    let mut guess;
    loop {
        let mut quit = false;
        guess = String::new();
        println!("Please input your guess or Q to quit.");
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        match guess.trim().parse::<i32>() {
        Ok(n) => println!("You guessed: {}", n),
        Err(_) => quit = should_quit(guess)
        }
        if quit {
            break;
        }
    }

    fn should_quit(guess: String) -> bool {
        let res = match guess.trim().to_lowercase().as_str() {
            "q" => true,
            _ => false
        };
        return res;

    }

    // println!("you guessed: {}", guess)
}