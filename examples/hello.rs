#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate vogue;

use std::env::consts::*;
use vogue::app::start;
use vogue::response::content::Html;

#[get("/")]
fn hello() -> Html<String> {
    Html(format!(
        r#"
    <head>
        <link rel="icon" href="https://img.icons8.com/cotton/1024/000000/ghost.png" />
        <title>Hello, world!</title>
        <style>
            body {{
                font-family: SFUIDisplay-Regular, Helvetica Neue, system-ui, Segoe UI, Helvetica, Arial, sans-serif;
                color: rgb(38, 45, 54);
            }}

            .centered {{
               position: absolute;
               top: 50%;
               left: 50%;
               transform: translate(-50%, -50%);
               text-align: center;
            }}

            .box {{
                border: none;
                border-radius: 1rem;
                padding: 1em 1em;
                box-shadow: 0px 2px 10px 1px rgba(0, 0, 0, 0.25);
            }}
        </style>
    </head>
    <body>
        <div class="centered">
            <img src="https://img.icons8.com/cotton/64/000000/ghost.png" />
            <h1>Welcome to Vogue!</h1>
            <div class="box">
            <h3>OS Info</h3>
            <p>Arch: {}</p>
            <p>DLL extension: {}</p>
            <p>EXE extension: {}</p>
            <p>Family: {}</p>
            <p>Operating system: {}</p>
            </div>
        </div>
    </body>
    "#,
        ARCH, DLL_EXTENSION, EXE_EXTENSION, FAMILY, OS
    ))
}

fn main() {
    start("HELLO_APP", routes![hello]);
}
