use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Error};

// erstellt einen privaten struct Guess, der in sich eine intiger mit dem "namen value hat
struct Guess{
    value: i32,
}


impl Guess{
    //fügt zu einem Guess struct die funktion hinzu, dass alle nummern, die in Guess eingetragen werden zwischen 1 und 100 sein müssen 
    //falls die Nummer nicht zischen 1 und 100 ist wird ein Err zurückgegeben
    pub fn new(value: i32) -> Result<Guess, Error>{
        if value < 1 || value > 100 {
            return Err(Error::new(io::ErrorKind::Other,format!("The value must be between 1 and 100, got {}",value)));
        }
        Ok(Guess { value })
    }

    //fügt eine funktion hinzu die value zurück gibt    
    pub fn value(&self) -> i32{
        self.value
    }
}
fn main() {
    println!("Guess the number between 1 and 100!");
    // generiert eine zufällige Nummer zwischen 1 und 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // eine simple loop aus der man nur mit break rauskommt(=while(true))
    loop {
        println!("Please input your guess.");
        
        //erstellt einen leeren veränderbaren String 
        let mut guess = String::new();
        
        //liest den Konsolen input aus und set guess zu dem output,
        //wenn es zu einen Fahler kommt gibt es einen panic mit dem Err failed to read line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        
        //guess wird neu definiert zu einem Guess struct
        //Wenn das Resultat von guess(String).trim().parse() einen Ok() mit einer i32 zurückgibt,
        //dann wird die funktion new von Guess ausgeführt. Bei einem gelingen wird guess auf diesen wert gesetzt. Andernfalls wird der Error ausgegeben und continue; genutzt.
        //Wenn guess.trim().parse() kein OK mit einer i32 zurückgibt wird der Spieler gebeten eine nummer einzugeben und continue; genutzt.
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

        //Wenn guess kleiner als secret_number ist wird das ausgegeben
        //Wenn guess größer als secret_number ist wird das ausgegeben
        //Wenn guess gleich der secret_number ist wird das spiel mit der nachricht you win beendet
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
