#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::tokio::time::{ sleep, Duration };
use rocket::serde::json::{ Json };

extern crate universal;
use universal::{ sender, receiver };

use std::env;

#[get("/")]
async fn index() -> String {
    sleep(Duration::from_secs(1)).await;

    "Hello World".into()
}

#[catch(404)]
async fn global_not_found(req: &Request<'_>) -> String {
    format!("not found uri: {}", req.uri())
}

#[post("/yuque/slack", format = "json", data = "<data>")]
async fn yuque_to_slack(data: Json<sender::yuque::Yuque>) -> String {
    receiver::slack::send(data.0).await
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"));
    
    let root = match env::var("MOUNT_ROOT") {
        Ok(s) => s,
        Err(_) => "/".into(),
    };
    
    rocket::custom(figment)
        .register("/", catchers![global_not_found])
        .mount(root, routes![index, yuque_to_slack])
}
