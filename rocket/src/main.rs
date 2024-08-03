use std::{io, time::Duration};

use rocket::tokio::{task::spawn_blocking, time::sleep};

#[macro_use]
extern crate rocket;

/// Dynamic Paths
#[get("/hello/<name>/<age>/<cool>")]
async fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::new(seconds, 0)).await;
    format!("Wait for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let r = rocket::build();
    let m = r.mount("/", routes![hello, delay, blocking_task]);
    m.launch().await?;

    Ok(())
}
