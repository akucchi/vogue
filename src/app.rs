use super::chrome::Chrome;
use rocket;
use rocket::config::Environment;
use rocket::response::content;
use rocket::Config;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[get("/vogue.js")]
fn vogue_js() -> content::JavaScript<&'static str> {
    content::JavaScript(include_str!("vogue.js"))
}

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
        .mount("/vogue/", routes![vogue_js])
        .launch();
}
