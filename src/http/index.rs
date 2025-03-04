use axum::{Router, extract::State, response::Html, routing::get};
use axum_macros::debug_handler;

use crate::Config;

pub(crate) fn router() -> Router<Config> {
    Router::new().route("/", get(index))
}

#[debug_handler]
async fn index(State(config): State<Config>) -> Html<String> {
    let files = std::fs::read_dir(config.markdown_dir).unwrap();
    let mut list = String::new();

    for file in files.flatten() {
        let path = file.path();
        if let Some(name) = path.file_stem() {
            if let Some(name) = name.to_str() {
                list.push_str(&format!("<li><a href='/prefix/{}'>{}</a></li>", name, name));
            }
        }
    }

    Html(format!(
        "<html><head><link rel='stylesheet' href='/theme/{}/main.css'></head><body><h1>Wiki</h1><ul>{}</ul></body></html>",
        config.theme, list
    ))
}
