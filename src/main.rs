#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;
extern crate rocket_contrib;

use rocket::get;
use rocket::routes;
use rocket::response::content;

// http://localhost:8000/post/your-post-name
#[get("/post/<post_name>")]
fn post(post_name: String) -> content::Html<String> {
    let file_path = format!("posts/{}.md", post_name);
    let markdown_text = std::fs::read_to_string(file_path).unwrap();
    let parser = pulldown_cmark::Parser::new(&markdown_text);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);
    content::Html(html_text)
}

fn main() {
    rocket::ignite().mount("/", routes![post]).launch();
}
