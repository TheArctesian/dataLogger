use promptly::{prompt, prompt_default};
use chrono::{DateTime, Utc};

fn main() {

    
    let date : DateTime<Utc> = Utc::now();
    let date_string = date.format("%Y-%m-%d").to_string();
    let time_string = date.format("%H:%M:%S").to_string();
    println!("Welcome to the Activity Tracker!");
    println!("Today is {}", date.format("%A, %B %d, %Y"));
    println!("Time is {}", date.format("%H:%M:%S"));
    // Activity related input
    let mut activity : String = prompt("Activity ").unwrap();
    let date : DateTime<Utc> = Utc::now();
    let mut timespent : i32 = prompt("Time Spent (mins)").unwrap();
    let mut location : String = prompt("Location").unwrap();
    // Emotional Input
    let mut productivity : i32 = prompt("Productivity").unwrap();
    let mut stress : i32 = prompt("Stress").unwrap();
    let mut happiness : i32 = prompt("Happiness").unwrap();
    let mut energy : i32 = prompt("Energy").unwrap();
    let mut interest : i32 = prompt("Interest").unwrap();
    // Social Input
    let mut social_Interaction = prompt_default("Social Interaction", true);
    let mut social_Interaction_Type : String = prompt("Social Interaction Type").unwrap();
    let mut social_Interaction_People_Count : i32= prompt("Social Interaction People Count").unwrap();
    println!("{}", activity);
    println!("{}", date_string);
    println!("{}", time_string);
    println!("{}", timespent);
    println!("{}", location);
    println!("{}", productivity);
    println!("{}", stress);
    println!("{}", happiness);
    println!("{}", energy);
    println!("{}", interest);
    println!("{}", social_Interaction_Type);
    println!("{}", social_Interaction_People_Count);
    // post_activity(activity, time, timespent, location, productivity, stress, happiness, energy, interest, social_Interaction);

//    let mut two = String::new();
//    io:stdin()
//        .read_line(&mut two)
//        .expect("Failed to read line");
//
//    println!("You gu {}", two);
}


// fn post(activity: String, time: SystemTime, timespent: i32, location: String, productivity: i32, stress: i32, happiness: i32, energy: i32, interest: i32, social_Interaction: bool) {
//    println!("{}", activity);
//    println!("{}", timespent);
//    println!("{}", location);
//    println!("{}", productivity);
//    println!("{}", stress);
//    println!("{}", happiness);
//    println!("{}", energy);
//    println!("{}", interest);
//    println!("{}", social_Interaction);

