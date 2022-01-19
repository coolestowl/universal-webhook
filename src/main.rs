#[macro_use]
extern crate rocket;

use std::env;

mod router;
use router::{ index, global_not_found };

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
        .mount(root, routes![index])
}
