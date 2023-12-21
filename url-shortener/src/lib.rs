mod routes;

use routes::Router;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_url_shortener(req: Request) -> anyhow::Result<impl IntoResponse> {
    let router = Router::default()?;
    router.redirect(req)
}
