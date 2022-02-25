use std::io;
use std::time::{SystemTime};

fn main() {

    println!("Input your activity:");
    
    // Activity related input
    let mut activity = String::new();
    let mut time = SystemTime::now();
    let mut timespent = i32
    let mut location = String::new();
    // Emotional Input
    let mut productivity = i32;
    let mut stress = i32;
    let mut happiness = i32;
    let mut energy = i32;
    let mut interest = i32;
    io::stdin()
        .read_line(&mut activity)
        .expect("Failed to read line");

    println!("You guessednum: {}", activity);
//
//    let mut two = String::new();
//    io:stdin()
//        .read_line(&mut two)
//        .expect("Failed to read line");
//
//    println!("You gu {}", two);
}
