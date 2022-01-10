use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Error};

struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Result<Guess, Error>{
        if value < 1 || value > 100 {
            return Err(Error::new(io::ErrorKind::Other,format!("The value must be between 1 and 100, got {}",value)));
        }
        Ok(Guess { value })
    }

    // We use a getter so that the value of Guess is always between 1 and 100 and can not be bypassed by simple changing the value to something else
    pub fn value(&self) -> i32{
        self.value
    }
}
fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num){
                Ok(value) => value,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            },
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            } 
        };
        
       
        println!("You guessed: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}