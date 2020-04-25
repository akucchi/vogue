# Vogue

Vogue is a library for building Electron-like applications with Rust, HTML, CSS and JavaScript.

<p align="center"><img src="https://raw.githubusercontent.com/akucchi/vogue/master/images/preview.png" /></p>

## Hello, world!

First we need to write few imports:

```rust
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate vogue;

use vogue::app::start;
use vogue::response::content::Html;
```

Then we can create `hello` function, which is routed to `/` and returns HTML response:

```rust
#[get("/")]
fn hello() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
```

Finally, launch the Vogue application as follows:

```rust
fn main() {
    start("HELLO_APP", routes![hello]);
}
```

`"HELLO_APP"` makes your app unique, so other Vogue apps won't show your's app content.

`routes![hello]` is basically list of routes for web server. Place all `#[get(...)]` functions here.
