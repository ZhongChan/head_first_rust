#![warn(clippy::all)]
use handle_errors::return_error;
use routes::answer::add_answer;
use routes::question::{add_question, delete_question, get_questions, update_question};
use store::Store;
use warp::filters::cors::Builder;
use warp::http::Method;
use warp::Filter;

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    //log init
    log_init();

    // wrap log custom
    let log = warp::log::custom(|info| {
        // Use a log macro, or slog, or println, or whatever!
        log::info!(
            "{} {} {} {:?} from {} whith {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });

    //fake database
    let store = Store::new();
    let store_fileter = warp::any().map(move || store.clone());

    // request id
    let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    // create a path Filter
    let path_hello = warp::path("hello").map(|| warp::reply::html("Hello, Wrap Filter!"));

    // Get first JSON response
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_fileter.clone())
        .and(id_filter)
        .and_then(get_questions);

    // Add question
    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    // Update qestion
    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    // Delete question
    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and_then(delete_question);

    // Add answer
    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and(warp::body::form())
        .and_then(add_answer);

    // Routes
    let routes = path_hello
        .or(get_questions)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(get_cors())
        .with(log)
        .recover(return_error);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// get_cors
/// ```
/// curl -X OPTIONS 127.0.0.1:3030/questions \
/// -H "Access-Control-Request-Method: PUT" \
///-H "Access-Control-Request-Headers: content-type" \
///-H "Origin: https://not-origin.io" \
///--verbose
/// ```
fn get_cors() -> Builder {
    warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST])
}

/// Log init
/// # Example 1
/// `RUST_LOG=info cargo run`
///
/// # Example 2
/// `RUST_LOG=info cargo run 2>logs.txt`
///
/// * `1` -> `stdout`
/// * `2` -> `stderr`
///
fn log_init() {
    // env_logger::init();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    log::warn!("This is a warn!");
    log::info!("This is info!");
    log::error!("This is an error!");
}
