pub mod schema;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build().launch().await?;
    Ok(())
}
