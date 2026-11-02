use std::io::Write;
use std::io;
use std::fs::File;
use std::path::Path;
use chrono::prelude::*;



fn main() {
    // grabbing date for file name
    let date: String =  Local::now()
        .format("%d-%m-%Y")
        .to_string();
    let mut counter = 1;
    println!("{date}");
    // creating file will append data later
    let mut new_save = format!("/home/wiggly/Documents/Gabito/{date}({counter}).txt");
    while Path::new(&new_save).exists() {
        new_save = format!("/home/wiggly/Documents/Gabito/{date}({counter}).txt");
        counter += 1;
    }
    let mut file = File::create(&new_save)
        .expect("Failed to create file.");

    let mut choice = String::new();
    println!("Hello, and welcome to your new practice bot. Let's get started by selecting...");
    println!("1.Addition\n2.Subtraction");
    io::stdin()
    .read_line(&mut choice)
    .expect("Enter a valid option.");
    let choice = choice.trim().parse::<i8>();

    if choice == Ok(1) {
        println!("Okay, take your time answering the questions because you WILL be graded.");
        for _ in 1..=20 {
            let mut answer = String::new();
            let a: i32 = rand::random_range(1..=1000);
            let b: i32 = rand::random_range(1..=1000);
            let problem = a + b;
            println!("{a} + {b} = ?");
            io::stdin()
                .read_line(&mut answer).expect("Failed to provide input.");
            let answer = answer.trim().parse::<i32>();
            if answer == Ok(problem) {
                println!("Correct!");
                file.write_all(format!("{a} + {b} = {:?}  => Correct!\n", answer).as_bytes())
                    .expect("Failed to write to file.");
            }
            else {
                println!("Incorrect!");
                file.write_all(format!("{a} + {b} = {:?}  => Wrong!\n", answer).as_bytes())
                    .expect("Failed to write to file.");
            }
        }
    }
    else if choice == Ok(2) {
        println!("Okay, take your time answering the questions because you WILL be graded.");
            for _ in 1..=20 {
                let mut answer = String::new();
                let a: i32 = rand::random_range(51..=100);
                let b: i32 = rand::random_range(1..=50);
                let problem = a - b;
                println!("{a} - {b} = ?");
                io::stdin()
                    .read_line(&mut answer).expect("Failed to provide input.");
                let answer = answer.trim().parse::<i32>();
                if answer == Ok(problem) {
                    println!("Correct!");
                    file.write_all(format!("{a} - {b} = {:?}  => Correct!\n", answer).as_bytes())
                        .expect("Failed to write to file.");
                }
                else {
                    println!("Incorrect!");
                    file.write_all(format!("{a} - {b} = {:?}  => Wrong!\n", answer).as_bytes())
                        .expect("Failed to write to file.");
                }

            }
    }
}
