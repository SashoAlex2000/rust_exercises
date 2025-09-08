use std::io;
use std::cmp::Ordering;

// to use this -> add rand = "0.8.5" to te Cargo.toml file and then run cargo build
use rand::Rng;


fn main() {

    println!("Guessing game!");
    
    // i32 -> signed 32 bit integer - can be negative -> this is the default - however I get and error with this
    // let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        
        let mut guess = String::new();
        
        // .read_line puts whatever the user entered into the passed variable, but it also 
        // returns a Results value, which is an enumeration
        // https://doc.rust-lang.org/std/result/enum.Result.html
        // use expect to "catch" if there is an error (just crash the program for now)
        // there will be a warning if there is an unhandled error possiblity 
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");
            println!("You guessed: {guess}");

        // Error handing - if non-number is entered:
        // thread 'main' panicked at src/main.rs:32:43:
        // Please type a number!: ParseIntError { kind: InvalidDigit }

        // We override the original `guess` variable - its called shadowing here 
        // I guess that if it was not `mut` we would not be able to?
        // This will not work as a string - the comparsion to the secret number will fail
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // This is the Error handled version:
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error parsing number!: {guess}");
                continue;
            },
        };
        
        // Ordering is an Enum from std
        // It has 3 variant results from comparing 2
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
