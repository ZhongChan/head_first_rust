use std::{io, path::PathBuf, time::Duration};

use rocket::{
    fs::{relative, FileServer, NamedFile},
    tokio::{task::spawn_blocking, time::sleep},
};

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

///  Multiple Segments
#[get("/page/<path..>")]
async fn get_page(path: PathBuf) -> Option<NamedFile> {
    let mut static_path = PathBuf::from("static");
    static_path.push(path);

    NamedFile::open(static_path).await.ok()
}

/// 启动函数
/// * (1): 将 static 文件夹中的内容挂载到 /static 路径下。
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let r = rocket::build();
    let r = r
        .mount("/", routes![hello, delay, blocking_task, get_page])
        .mount("/static", FileServer::from(relative!("static"))); // (1)

    r.launch().await?;

    Ok(())
}
