extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    // println!("Hello, world!");
    // guessing_game();
    //ownership();
    test_area();
}

fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("The secret is {}", secret_number);
    loop {

        println!("Input your guess:");
        let mut user_input= String::new();

        io::stdin().read_line(&mut user_input)
            .expect("Failed to read input!");

        let user_input:u32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => break,
        };

        println!("You selected {}", user_input);
        match user_input.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You guessed correct!🎉🎊");
                break;
            },
        }
    }
}

/// # Ownership Testing
/// Here I am testing several things related to ownership in RUST
fn ownership(){
    let mut s = String::from("Hello");
    s.push_str(", Anique");
    println!("{}", s);

    s = takes_and_gives(s);
    println!("Given Back: {}", s);
}

fn takes_and_gives(mut s: String) -> String {
    s.push_str(", my master");
    s
}

/// Methods and structs

#[derive(Debug)]
struct Rectangle{
    width: usize,
    height: usize
}

impl Rectangle{
    fn area(&self) -> usize {
        self.height*self.width
    }
}

fn test_area(){
    let rect = Rectangle {width: 20, height:30};
    println!("Rect: {:?} Area: {}", rect, rect.area());
}