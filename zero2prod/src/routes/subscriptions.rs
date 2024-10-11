use chrono::Utc;
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

    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    log::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );

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
    .await;

    match result {
        Ok(_) => {
            log::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
