use promptly::{prompt, prompt_default};
use chrono::{DateTime, Utc};

fn main() {
    
    let date = Utc::now().naive_utc() + chrono::Duration::hours(8); // Set timezone to UTC +8
    let date_string = date.format("%Y-%m-%d").to_string(); // Format date to string
    let time_string = date.format("%H:%M:%S").to_string();  // Format time to string
    println!("Welcome to the Activity Tracker!"); 
    println!("Today is {}", date.format("%A, %B %d, %Y"));
    println!("Time is {}", date.format("%H:%M:%S"));
    // Activity related input
    let mut activity : String = prompt("Activity").unwrap(); // Get activity name
    let mut topic : String = prompt("Topic/Project").unwrap(); // Get topic/project name
    let mut timespent : i32 = prompt("Time Spent (mins)").unwrap(); // Get time spent
    let mut location : String = prompt("Location").unwrap(); // Get location
    // Emotional Input
    let mut productivity : i32 = prompt("Productivity").unwrap(); // Get productivity
    let mut stress : i32 = prompt("Stress").unwrap(); // Get stress
    let mut happiness : i32 = prompt("Happiness").unwrap(); // Get happiness
    let mut energy : i32 = prompt("Energy").unwrap(); // Get energy
    let mut interest : i32 = prompt("Interest").unwrap(); // Get interest
    // Social Input
    let social_interaction = prompt_default("Social interactionc", true).unwrap(); // Get social interaction bool

    postToDB(activity, date_string, time_string, timespent, location, productivity, stress, happiness, energy, interest, social_interaction);

}

 fn postToDB(Activity : String, 
    Date : String, 
    Time : String, 
    TimeSpent : i32, 
    Location : String, 
    Productivity : i32, 
    Stress : i32, 
    Happiness : i32, 
    Energy : i32, 
    Interest : i32, 
    SocialInteraction : bool) {
            println!("{}", Activity);
    println!("{}", Date);
    println!("{}", Time);
    println!("{}", TimeSpent);
    println!("{}", Location);
    println!("{}", Productivity);
    println!("{}", Stress);
    println!("{}", Happiness);
    println!("{}", Energy);
    println!("{}", Interest);
    println!("{}", SocialInteraction);

}
    






















//    let mut two = String::new();
//    io:stdin()
//        .read_line(&mut two)
//        .expect("Failed to read line");
//
//    println!("You gu {}", two);


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

