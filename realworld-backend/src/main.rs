pub mod schema;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    realworld_backend::rocket().launch().await?;
    Ok(())
}
