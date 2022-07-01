extern crate colorful;

use colorful::{Color, Colorful};

use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Guess{
    value : i32
}

impl Guess {
    pub fn new(number : i32) -> Guess{
        if !(1..100).contains(&number) {
            panic!("{}", "YOUR NUMBER NEEDS TO BE BETWEEN 0 AND 100!".color(Color::Red3b));

        }

         Guess{ value : number }
        
    }
    fn value(&self) -> i32{
        self.value
    }
}
fn main() {
   
        let guessing_game = "GUESSING GAME!!";
        let greater = "Too Big!!! the right answer was";
        let less = "Too small!!!! the answer was";
        let won = "Whoooaaa you WON!!!";
        println!("{}", guessing_game.color(Color::BlueViolet));
    loop {
        println!("Please enter you guess......");
        let secret_number = rand::thread_rng().gen_range(0..100);

        let mut guess = String::new();
        io::stdin() 
            .read_line(&mut guess)
            .expect("FAILED TO READ THE LINE!");
        println!("YOU HAVE GUESSED {}", guess);

        let guess: i32 = guess.trim().parse()
        .expect("PLEASE TYPE A NUMBER!!!");
        let guessed = Guess::new(guess);

            match guessed.value().cmp(&secret_number) {
            Ordering::Equal => {
                println!(
                    "{}\n your guess = {} \n the answer = {}",
                    won.color(Color::Green),
                    &guess,
                    secret_number
                );
                break;
            }
            Ordering::Greater => println!("{} {}", greater.color(Color::Red), secret_number),
            Ordering::Less => println!("{} {}", less.color(Color::Red), secret_number),
        }
    }
}
