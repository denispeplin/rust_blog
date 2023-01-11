#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;
extern crate rocket_contrib;

use rocket::get;
use rocket::post;
use rocket::request::Form;
use rocket::response::content;
use rocket::response::Redirect;
use rocket::routes;
use rocket::FromForm;

extern crate chrono;
use chrono::prelude::*;

use std::fs;
use std::io::Write;

extern crate slug;
use self::slug::slugify;

#[derive(FromForm)]
struct NewPost {
    title: String,
    content: String,
}

#[derive(FromForm)]
struct ExistingPost {
    content: String,
}

// http://localhost:8000/new
#[get("/new")]
fn new_post_form() -> content::Html<String> {
    let html = r#"
    <form action="/new" method="post">
        <label for="title">Post title</label>
        <input type="text" id="title" name="title" required>
        <br>
        <label for="content">Post content</label>
        <textarea id="content" name="content" rows="10" required></textarea>
        <br>
        <input type="submit" value="Create Post">
    </form>
    "#;
    content::Html(html.into())
}

#[post("/new", data = "<post_form>")]
fn create_post(post_form: Form<NewPost>) -> Redirect {
    let current_date = Local::now().to_string();
    let file_name = slugify(&post_form.title);
    let file_path = format!("posts/{}.md", file_name);
    let mut file = fs::File::create(file_path).unwrap();
    let _ = file.write_all(
        format!(
            "---\ntitle: {}\ncreated_at: {}\n---\n\n{}",
            post_form.title, current_date, post_form.content
        )
        .as_bytes(),
    );
    Redirect::to(format!("/post/{}", file_name))
}

// http://localhost:8000/post/your-post-name
#[get("/post/<post_name>")]
fn post(post_name: String) -> content::Html<String> {
    let file_path = format!("posts/{post_name}.md");
    let markdown_text = std::fs::read_to_string(file_path).unwrap();
    let parser = pulldown_cmark::Parser::new(&markdown_text);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);
    content::Html(html_text)
}

#[get("/posts")]
fn posts() -> content::Html<String> {
    let dir = "posts";
    let post_files = std::fs::read_dir(dir).unwrap();

    let mut post_list = String::new();
    post_list.push_str("<ul>");

    for file in post_files {
        let file = file.unwrap();
        let file_name = file.file_name().to_str().unwrap().to_owned();
        post_list.push_str(&format!(
            "<li><a href='/post/{}'>{}</a></li>",
            file_name.split('.').next().unwrap(),
            file_name
        ));
    }

    post_list.push_str("</ul>");
    content::Html(post_list)
}

// http://localhost:8000/edit/your-post-name
#[get("/edit/<post_name>")]
fn edit_post_form(post_name: String) -> content::Html<String> {
    let file_path = format!("posts/{post_name}.md");
    let post_content = std::fs::read_to_string(file_path).unwrap();
    let html = format!(
        r#"
    <form action="/edit/{post_name}" method="post">
        <label for="content">Post content</label>
        <textarea id="content" name="content" rows="10">{post_content}</textarea>
        <br>
        <input type="submit" value="Save">
    </form>
    "#
    );
    content::Html(html)
}

#[post("/edit/<post_name>", data = "<post_form>")]
fn update_post(post_name: String, post_form: Form<ExistingPost>) -> Redirect {
    let file_path = format!("posts/{post_name}.md");
    let _ = std::fs::write(file_path, &post_form.content);
    Redirect::to(format!("/post/{post_name}"))
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                new_post_form,
                create_post,
                post,
                posts,
                edit_post_form,
                update_post
            ],
        )
        .launch();
}
