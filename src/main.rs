use axum::{extract::Path, response::Html, routing::get, Router};
use pulldown_cmark::{html, Options, Parser};
use std::{fs, path::PathBuf};

const MARKDOWN_DIR: &str = "./markdown";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/wiki/{page}", get(serve_markdown))
        .route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let files = fs::read_dir(MARKDOWN_DIR).unwrap();
    let mut list = String::new();

    for file in files.flatten() {
        let path = file.path();
        if let Some(name) = path.file_stem() {
            if let Some(name) = name.to_str() {
                list.push_str(&format!("<li><a href='/wiki/{}'>{}</a></li>", name, name));
            }
        }
    }

    Html(format!("<h1>Remarkable</h1><ul>{}</ul>", list))
}

async fn serve_markdown(Path(page): Path<String>) -> Html<String> {
    let path = PathBuf::from(MARKDOWN_DIR).join(format!("{}.md", page));
    if let Ok(content) = fs::read_to_string(path) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Html(format!("<html><body>{}</body></html>", html_output))
    } else {
        Html("<h1>Page not found</h1>".to_string())
    }
}
