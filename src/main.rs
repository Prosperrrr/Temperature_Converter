//  Hello World, this is my first proper project while learning rust from the rust book, just decided to push to git because why not? lol
//  let's cook!
//  so i'm building a Temperature converter, I'd call it "The thermal Calibration Unit" so it doesn't bore me too much.
//  essentially I'm building a program that converts Farenheit to Celsius and reverse (Celsius to Farenheit)
//  This is the math to be used C = (F - 32) * 5/9 (celsius to Farenheit)

use std::io;
fn main() {
    println!("Hi, let's convert!");
    
    loop{
        println!("Input the value you want to convert");
        let mut user_input = String::new();

        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the line");

        let user_input :f64 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input an actual number");
                continue;
            }
        };

        let conversion_to_celsius: f64 = user_input - 32.0 * 5.0/9.0; 
        println!(
            "Conversion complete
            Fahrenheit = {}
            Celsius = {}
            ", user_input, conversion_to_celsius
        );
    }
}
