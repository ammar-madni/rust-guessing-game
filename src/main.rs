use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\nGuess a number between 1 and 100.\n");

    let target_number = rand::thread_rng().gen_range(1..101);
    println!("The target number is: {}", target_number);

    println!("Input your guess:");

    let mut guess = String::new();

    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess.trim().parse().expect("Must be a number!");
        
        println!("Your guess: {}", guess);

        match guess.cmp(&target_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Well done! Your guess was correct.");
                break;
            }
        }
    }
}
