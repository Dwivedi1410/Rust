use std::io;
// To obtain the user input and to print the output we have to use io input output library.(comes from std library)

use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guessing Game");

    // here i am generating a random number in the range of 1 to 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("This is the secret number generated {secret_number}");
    loop {
        println!("Enter the number you have choosen");

        // here i am creating a new mutable variable guess of type string
        let mut guess = String::new();

        //here i am taking input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // here i am converting the string to unsigned 32 bit, here parse method convert the guess to some other type but we have to explicitly mention the type (let guess: u32) here we are mentioning the type as unsigned 32 bit.
        // parse will convert only those strings that are convertible to number else it will give err which will be handled by the expect similar to  the read_line.
        // let guess: u32 = guess.trim().parse().expect("Enter the valid number");  this is one method to convert the string to number and to handle the error we are using the expect.

        // in this method we are using the match method so if err occurs we will continue the loo[ and if everything will we ok then we will compare the guess and secret_number.
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // comapring guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
        }

        println!("you guessed: {guess}");
    }
}
