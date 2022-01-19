use rocket::Request;
use rocket::tokio::time::{ sleep, Duration };
use rocket::serde::json::{ Json };

use crate::sender::{ yuque, diy };
use crate::receiver::slack;

#[get("/")]
pub async fn index() -> String {
    sleep(Duration::from_secs(1)).await;

    "Hello World".into()
}

#[catch(404)]
pub async fn global_not_found(req: &Request<'_>) -> String {
    format!("not found uri: {}", req.uri())
}

#[post("/yuque/slack", format = "json", data = "<req>")]
pub async fn yuque_to_slack(req: Json<yuque::Yuque>) -> String {
    slack::send(req.0).await
}

#[post("/diy/slack", format = "json", data = "<req>")]
pub async fn diy_to_slack(req: Json<diy::Slice>) -> String {
    slack::send(req.0).await
}
