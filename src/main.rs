use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    let inu = Animal {
        name: String::from("INU"),
        voice: String::from("WAN"),
        heavy: 32,
        hight: 109,
    };


    let inu_wan = Animal {
        name: String::from("INU-WAN"),
        hight: 129,
        ..inu
    };

    println!("Next! {}!! {}!!", inu_wan.name, inu_wan.voice);

    let neko = String::from("NECO-CHAN!");

    // let neko = "WANCO-CHAN!";

    let res = slice(&neko);

    println!("This is a {}", res);


    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");
    
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };    
    
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

fn slice(s: &str) -> &str {
    &s[..3]
}

struct Animal {
    name: String,
    hight: u32,
    heavy: u32,
    voice: String,
}