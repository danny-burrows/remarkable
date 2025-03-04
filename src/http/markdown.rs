use axum::{
    Router,
    extract::{Path, State},
    response::Html,
    routing::get,
};
use axum_macros::debug_handler;
use pulldown_cmark::{Options, Parser};

use crate::Config;

pub(crate) fn router() -> Router<Config> {
    Router::new().route("/prefix/{page}", get(serve_markdown))
}

#[debug_handler]
async fn serve_markdown(State(config): State<Config>, Path(page): Path<String>) -> Html<String> {
    let path = config.markdown_dir.join(format!("{}.md", page));
    if let Ok(content) = std::fs::read_to_string(path) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        Html(format!(
            "<html><head><link rel='stylesheet' href='/theme/{}/main.css'></head><body>{}</body></html>",
            config.theme, html_output
        ))
    } else {
        Html(format!(
            "<html><head><link rel='stylesheet' href='/theme/{}/main.css'></head><body><h1>Page Not Found!</h1></body></html>",
            config.theme
        ))
    }
}
