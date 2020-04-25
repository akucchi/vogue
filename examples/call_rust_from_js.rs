#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate vogue;

use vogue::app::start;
use vogue::response::content::Html;

#[get("/add/<x>/<y>")]
fn add(x: i64, y: i64) -> String {
    format!("{} + {} = {}", x, y, x + y)
}

#[get("/")]
fn home() -> Html<&'static str> {
    Html(r#"
        <title>Call Rust from JS</title>
        <h1 id="result"></h1>
        <script>
            fetch("/add/5/8")
                .then(response => response.text())
                .then(data => {
                    console.log(data);
                    document.getElementById("result").innerText = data;
                });
        </script>
    "#)
}

fn main() {
    start("CALL_RUST_FROM_JS", routes![add, home]);
}
