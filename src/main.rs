//  Hello World, this is my first proper project while learning rust from the rust book, just decided to push to git because why not? lol
//  let's cook!
//  so i'm building a Temperature converter, I'd call it "The thermal Calibration Unit" so it doesn't bore me too much.
//  essentially I'm building a program that converts Farenheit to Celsius and reverse (Celsius to Farenheit)
//  This is the math to be used C = (F - 32) * 5/9 (celsius to Farenheit)

use std::io;
fn main() {
    println!("Hi, let's convert!");
    println!("Do you want to convert Farenheit or Celsius?");

    'mainevent: loop{
        'CFcalulation: loop{
        let mut user_choice = String::new();

    io::stdin()
    .read_line(&mut user_choice)
    .expect("Failed to read the line");

    let user_choice = user_choice.trim();

    if user_choice == "Farenheit" {
        println!("Okay, Input Value in Farenheit");
        loop{
        let mut user_input = String::new();

        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the line");

        let user_input :f64 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Error, Please input an actual number");
                continue;
            }
        };

        let conversion_to_celsius: f64 = (user_input - 32.0) * 5.0/9.0; 
        println!(
            "Conversion complete
            Fahrenheit = {}
            Celsius = {}
            ", user_input, conversion_to_celsius
        );
        break 'CFcalulation;

        // println!("Type 'quit' to exit or press Enter to calculate another");

        // loop{
        // let mut user_response = String::new();

        // io::stdin()
        // .read_line(&mut user_response)
        // .expect("Failed to read the line");

        // let user_response = user_response.trim();
        // // println!("User responded with {}", user_response);
        // if user_response == "quit"{
        //     println!("Okay, thank you for using me GG's");
        //     break 'Fcalculate;
        // }else if user_response == ""{
        //     println!("Okay, Let's go again. Input the value you want to convert, from Farenheit to Celsius");
        //     continue 'Fcalculate;
        // }else{
        //     println!("Wrong, Please type 'quit' to end or press Enter to continue");
        // }
        // }
    }
    }else if user_choice == "Celsius" {
        println!("Input Value in Celsius");
        loop{
        let mut user_input = String::new();

        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the line");

        let user_input :f64 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Error, Please input an actual number");
                continue;
            }
        };

        let conversion_to_farenheit: f64 = (user_input * 9.0/5.0) + 32.0; 
        println!(
            "Conversion complete
            Celsius = {}
            Farenheit = {}
            ", user_input, conversion_to_farenheit
        );

        // println!("Type 'quit' to exit or press Enter to calculate another");

        // loop{
        // let mut user_response = String::new();

        // io::stdin()
        // .read_line(&mut user_response)
        // .expect("Failed to read the line");

        // let user_response = user_response.trim();
        // // println!("User responded with {}", user_response);
        // if user_response == "quit"{
        //     println!("Okay, thank you for using me GG's");
        //     break 'Ccalculate;
        // }else if user_response == ""{
        //     println!("Okay, Let's go again. Input the value you want to convert, from Farenheit to Celsius");
        //     continue 'Ccalculate;
        // }else{
        //     println!("Wrong, Please type 'quit' to end or press Enter to continue");
        // }
        // }
        break 'CFcalulation;
    }
    }else {
        println!("Please input either 'Farenheit' or 'Celsius'");
    }
    }
    println!("Type 'quit' to exit or press Enter to calculate another");

        loop{
        let mut user_response = String::new();

        io::stdin()
        .read_line(&mut user_response)
        .expect("Failed to read the line");

        let user_response = user_response.trim();
        // println!("User responded with {}", user_response);
        if user_response == "quit"{
            println!("Okay, thank you for using me GG's");
            break 'mainevent;
        }else if user_response == ""{
            println!("Okay, Let's go again. Do you want to convert Farenheit or Celsius?");
            continue 'mainevent;
        }else{
            println!("Wrong, Please type 'quit' to end or press Enter to continue");
        }
        }
    }
}
