use chrono::Utc;
use tracing::Instrument;
use uuid::Uuid;

use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// subscribe
// (2) Retriving a connection from the application state!
pub async fn subscribe(
    form: web::Form<FormData>,
    db_pool: web::Data<PgPool>, //(2)
) -> HttpResponse {
    let request_id = Uuid::new_v4();

    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!("Adding a new subscriber.",%request_id,subscriber_email = %form.email,subscriber_name = %form.name);

    // Using `enter` in an async function is a recipe for disaster!
    let _request_span_gurad = request_span.enter();
    // `_request_span_gurad` is dropped at the end of `subscribe`
    // That's when we "exit" the span

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    let result = sqlx::query!(
        r#"
              insert into subscriptions (id, email, name, subscribed_at)
              values ($1, $2, $3, $4)
              "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .instrument(query_span)
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
