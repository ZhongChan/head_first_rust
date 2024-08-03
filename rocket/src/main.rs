use std::{io, time::Duration};

use rocket::tokio::{task::spawn_blocking, time::sleep};

#[macro_use]
extern crate rocket;

#[get("/hello")]
async fn hello() -> &'static str {
    "Hello, world!"
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
