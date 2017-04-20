extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let rand_n = rand::thread_rng().gen_range(1, 101); //immutable randomized number to guess

    //println!("The Number is {}", rand_n); //Print to check number

    loop{
		    println!("Please input your guess.");

		    let mut guess = String::new(); //mutable guess and :: associates new of type string

		    io::stdin().read_line(&mut guess)
				.expect("Failed to read line.");

		    let guess: u32 = match guess.trim().parse(){
		    		Ok(num) => num,
				Err(_) => continue,
				};

		    println!("You guessed: {}", guess);

		    match guess.cmp(&rand_n){
				    Ordering::Less => println!("Too small"),
				    Ordering::Greater => println!("Too big"),
				    Ordering::Equal => {
				    	println!("Exactly, you win");
				    	break;}
		    }
    }
}
