use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");
    println!("Enter the range of the number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error Occured");
    let mn : u32 = match input.trim().parse() {
        Ok (num) => num,
        Err (_) => 10,
    };
    let rn = rand::thread_rng().gen_range(1..=mn);
    loop {
        println!("Guess a number");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error occured");
        let inum : u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match inum.cmp(&rn) {
            Ordering::Greater => println!("Too Great"),
            Ordering::Less => println!("Too Less"),
            Ordering::Equal=> {
                println!("You Won!");
                break;
            }
        }
    }
}
