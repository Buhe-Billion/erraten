use std::{io, cmp::Ordering};
use rand::Rng;

pub struct Guess { value: i32, }

impl Guess
{

    pub fn new
    (value: i32) -> Guess
    {
        if value < 1 || value > 100
        { panic!("Guess value must be between 1 and 100, but we got {value}."); }

        Guess { value }
    }

    pub fn value
    (&self) -> i32
    {self.value}

}

fn main
()
{
    println!("Rate die Zahl!");

    let geheim_zahl = rand::thread_rng().gen_range(1..=100);

    loop
    {

        println!("Bitte gib deine Schätzung ein.");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Fehler beim Lesen der Zeile");

        let guess: i32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => continue,
            };

        let guess = Guess::new(guess).value();

        println!("Du hast geschätzt: {guess}");

        match guess.cmp(&geheim_zahl)
        {

            Ordering::Less => println!("Zu klein!"),
            Ordering::Greater => println!("Zu groß!"),
            Ordering::Equal =>
                {
                    println!("Du hast gewonnen!");
                    break;
                }
        }

    }
}
