#![warn(clippy::all)]
use config::Config;
use handle_errors::return_error;
use routes::answer::add_answer;
use routes::authentications::{login, register};
use routes::question::{add_question, delete_question, get_questions, update_question_tokio_spawn};
use store::Store;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::filters::cors::Builder;
use warp::http::Method;
use warp::Filter;

mod apilayer;
mod routes;
mod store;
mod types;

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
struct Args {
    log_level: String,
    database_host: String,
    database_port: u16,
    database_name: String,
    port: u16,
}

#[tokio::main]
async fn main() {
    //init config
    let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();
    let config = config.try_deserialize::<Args>().unwrap();

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust_web_dev={},warp={}",
            config.log_level, config.log_level, config.log_level
        )
    });

    //fake database
    let store = Store::new(&format!(
        "postgres://{}:{}/{}",
        config.database_host, config.database_port, config.database_name
    ))
    .await;
    let store_fileter = warp::any().map(move || store.clone());

    // tracing
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span cloese.
        // This can be used to time our
        // route's durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // create a path Filter
    let path_hello = warp::path("hello").map(|| warp::reply::html("Hello, Wrap Filter!"));

    // Get first JSON response
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentications::auth())
        .and(warp::query())
        .and(store_fileter.clone())
        .and_then(get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_questions request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4()
            )
        }));

    // Add question
    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentications::auth())
        .and(store_fileter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    // Update qestion
    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentications::auth())
        .and(store_fileter.clone())
        .and(warp::body::json())
        .and_then(update_question_tokio_spawn);

    // Delete question
    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentications::auth())
        .and(store_fileter.clone())
        .and_then(delete_question);

    // Add answer
    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and(warp::body::form())
        .and_then(add_answer);

    // Add account
    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and(warp::body::json())
        .and_then(register);

    // login
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and(warp::body::json())
        .and_then(login);

    // Routes
    let routes = path_hello
        .or(get_questions)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(get_cors())
        .with(warp::trace::request())
        .recover(return_error);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;
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
