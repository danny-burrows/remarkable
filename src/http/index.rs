use askama::Template as _;
use axum::response::Html;
use axum::{Router, extract::State, routing::get};
use axum_macros::debug_handler;

use crate::Config;
use crate::http::LayoutTemplate;

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

    Html(
        LayoutTemplate {
            title: "Remarkable",
            theme: &config.theme,
            body: &format!("<h1>Remarkable</h1><ul>{list}</ul>"),
        }
        .render()
        .unwrap(),
    )
}
