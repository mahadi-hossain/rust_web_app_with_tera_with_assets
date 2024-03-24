use axum::response::Html;
use tera::Context;

use crate::tera_setup;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}



pub async fn home_page()-> Html<String> {
    let mut context = Context::new();
        context.insert("name", "Mahadia");

    let rendered_html: String = tera_setup::TEMPLATES
        .render("greet.html", &context)
        .expect("Failed to render template");

    Html(rendered_html)
}