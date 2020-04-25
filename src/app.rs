use super::chrome::Chrome;
use rocket;
use rocket::config::Environment;
use rocket::response::content;
use rocket::Config;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn start(app_id: &str, routes: Vec<rocket::Route>) {
    let mut hasher = DefaultHasher::new();
    hasher.write(app_id.as_bytes());

    let port = (hasher.finish() % 0x10000) as u16;

    let config = Config::build(Environment::Staging)
        .address("localhost")
        .port(port)
        .finalize()
        .unwrap();

    Chrome::new()
        .open(format!("http://localhost:{}", port).as_str())
        .unwrap();

    rocket::custom(config)
        .mount("/", routes)
        .launch();
}
