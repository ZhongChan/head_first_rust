#![warn(clippy::all)]
use clap::Parser;
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

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about=None)]
struct Args {
    #[clap(short, long, default_value = "warn")]
    log_level: String,
    #[clap(long, default_value = "localhost")]
    database_host: String,
    #[clap(long, default_value = "5432")]
    database_port: u16,
    #[clap(long, default_value = "rustwebdev")]
    database_name: String,
    #[clap(long, default_value = "3030")]
    port: u16,
}

/// Get more info use
///
/// # Use toml
/// `RUST_LOG=debug cargo run`
///
/// # Use args
/// `cargo run -- --port 8080`
///
///
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust_web_dev={},warp={}",
            args.log_level, args.log_level, args.log_level
        )
    });

    //fake database
    let store = Store::new(&format!(
        "postgres://{}:{}/{}",
        args.database_host, args.database_port, args.database_name
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

    // build info
    tracing::info!("Q&A service build ID {}", env!("RUST_WEB_DEV_VERSION"));

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], args.port)).await;
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
