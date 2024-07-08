use ch_02::Question;
use ch_02::QuestionId;
use std::str::FromStr;
use warp::Filter;

#[tokio::main]
async fn main() {
    // create a path Filter
    let path_hello = warp::path("hello").map(|| format!("Hello,Warp Filter!"));

    // Get first JSON response
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);
    let routes = path_hello.or(get_items);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").unwrap(),
        "First Question".to_string(),
        "Conent of question".to_string(),
        Some(vec!["faq".to_string(), "web".to_string()]),
    );
    Ok(warp::reply::json(&question))
}
