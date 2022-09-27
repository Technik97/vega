use axum::{
    routing::{get},
    Router, Extension, response::{Html},
};
use tera::{Tera, Context};
use std::{net::SocketAddr};

mod views;
use views::tera_config::get_tera;


#[tokio::main]
async fn main() {
    let tera:Tera = get_tera();

    let app = Router::new()
        .route("/hello", get(root))
        .route("/", get(index))
        .route("/sign_up", get(register))
        // .route("/styles.css", get(styles))
        .layer(Extension(tera));
        

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}


async fn index(
    Extension(tera): Extension<Tera>
) -> Html<String> {
    // let tera = get_tera();
    let context = Context::new();
    Html(tera.render("index.html", &context).unwrap())
}

async fn register(
    Extension(tera): Extension<Tera>
) -> Html<String> {
    let context = Context::new();
    Html(tera.render("register.html", &context).unwrap())
}
