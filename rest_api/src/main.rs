#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

// Test async
// it dost not
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> &'static str {
    "test"
}


// Mounts and builds api 
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/", routes![test])
        .mount("/", routes![delay])
}
