use axum::Router;
use axum::extract::MatchedPath;
use axum::extract::Request;
use std::time::Duration;
use tracing::{info, info_span};

use tokio::signal;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::Config;
use crate::error::Result;
use crate::error::fallback_handler_404;

mod index;
mod markdown;

#[derive(askama::Template)]
#[template(path = "layout.html")]
pub(crate) struct LayoutTemplate<'a> {
    title: &'a str,
    theme: &'a str,
    body: &'a str,
}

fn base_router() -> Router<Config> {
    Router::new()
        .merge(index::router())
        .merge(markdown::router())
}

pub async fn serve(config: Config) -> Result<()> {
    let themes_dir = std::env::current_dir()?.join("themes");
    let app = base_router()
        .nest_service("/theme", ServeDir::new(&themes_dir))
        .layer(
            TraceLayer::new_for_http()
                // Create our own span for the request and include the matched path. The matched
                // path is useful for figuring out which handler the request was routed to.
                .make_span_with(|request: &Request| {
                    let method = request.method();
                    let uri = request.uri();
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!("http_request", ?method, ?uri, matched_path)
                })
                // By default `TraceLayer` will log 5xx responses but we're doing our specific
                // logging of errors so disable that
                .on_failure(()),
        )
        .layer(
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10)),
        )
        .with_state(config)
        .fallback(fallback_handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
