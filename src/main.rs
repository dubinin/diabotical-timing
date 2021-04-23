use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io::{self, Write};

enum ParseErr {
    Error,
    Exit,
}

#[derive(Debug)]
enum Item {
    Armor,
    Mega,
}

impl Distribution<Item> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Item {
        match rng.gen_range(0..=1) {
            0 => Item::Armor,
            1 => Item::Mega,
            _ => Item::Mega,
        }
    }
}

fn main() {
    loop {
        let moment = get_rnd_time();
        let item: Item = rand::random();
        match get_user_input(&moment, &item) {
            Ok(value) => {
                let waiting = match item {
                    Item::Mega => 35,
                    Item::Armor => 25,
                };
                let correct_value = (moment + waiting) % 60;
                if correct_value == value {
                    println!("Correct!");
                } else {
                    println!("Should be {}", correct_value);
                }
            }
            Err(err) => match err {
                ParseErr::Error => {
                    println!("Some error occurs.");
                    break;
                }
                ParseErr::Exit => {
                    println!("Exit program.");
                    break;
                }
            },
        }
    }
}

fn get_user_input(moment: &u8, item: &Item) -> Result<u8, ParseErr> {
    print!("{} sec. {:?}, next will be at? ", moment, item);
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            let value = user_input.trim().to_lowercase();
            if value.eq("exit") {
                Err(ParseErr::Exit)
            } else {
                match value.parse::<u8>() {
                    Ok(number) => Ok(number),
                    Err(_) => Err(ParseErr::Error),
                }
            }
        }
        Err(_) => Err(ParseErr::Error),
    }
}

fn get_rnd_time() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..60)
}
