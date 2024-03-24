
use axum::{extract::{Extension, Path, Query, }, handler::HandlerWithoutStateExt, http::StatusCode, response::Html, routing::get, Error, Router};
use serde::Serialize;
use tera::{Context, Tera};
use axum_template;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
mod tera_setup;
mod controllers;



#[tokio::main]
async fn main() {

    tera_setup::init_templates();
    // build our application with a route
    let app = using_serve_dir_with_handler_as_service()
    .route("/", get(controllers::home_page))
    .route("/hello", get(controllers::handler))
    .nest_service("/assets", ServeDir::new("src/assets"))
    ;

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



fn using_serve_dir_with_handler_as_service() -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found, Mama:(")
    }

    // you can convert handler function to service
    let service = handle_404.into_service();

    let serve_dir = ServeDir::new("assets").not_found_service(service);

    Router::new()
        
        .fallback_service(serve_dir)
}