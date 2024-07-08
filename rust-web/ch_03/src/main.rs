use warp::Filter;

#[tokio::main]
async fn main() {
    // create a path Filter
    let path_hello = warp::path("hello").map(|| format!("Hello,Warp Filter!"));

    // start the server and pass the route filter to it
    warp::serve(path_hello).run(([127, 0, 0, 1], 3030)).await;
}
