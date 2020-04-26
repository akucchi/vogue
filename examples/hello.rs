#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate vogue;

use std::env::consts::*;
use vogue::app::start;
use vogue::response::content::Html;

#[get("/")]
fn hello() -> Html<String> {
    Html(format!(
        include_str!("hello.html"),
        ARCH, DLL_EXTENSION, EXE_EXTENSION, FAMILY, OS
    ))
}

fn main() {
    start("HELLO_APP", routes![hello]);
}
