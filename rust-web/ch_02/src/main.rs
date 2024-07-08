use warp::Filter;
mod tests;

/// # `String` and `&str`
/// A quick summary:
/// * If you need own and modify the text,create a `String` type.
/// * Use `&str` when you need only a view of the underlying text.
/// * When creating new data type via a struct,you typically create `String` field types.
/// * When passing strings/text to a function,you usually use `&str`.

#[tokio::main]
async fn main() {
    let hello = warp::get() // (1)
        .map(|| warp::reply::html("Hello, World!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 1337)) // (2)
        .await; // (3)
}
