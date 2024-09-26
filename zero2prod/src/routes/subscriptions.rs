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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
