use rocket::Request;
use rocket::tokio::time::{ sleep, Duration };
use rocket::serde::json::{ Json };

#[get("/")]
pub async fn index() -> String {
    sleep(Duration::from_secs(1)).await;

    "Hello World".into()
}

#[catch(404)]
pub async fn global_not_found(req: &Request<'_>) -> String {
    format!("not found uri: {}", req.uri())
}
