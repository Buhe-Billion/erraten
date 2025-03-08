use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn
main
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

        let guess: u32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => continue,
            };

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
