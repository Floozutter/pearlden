pub struct App {}

impl App {
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = axum::Router::new()
            .route("/", axum::routing::get(handler))
            .layer(tower_http::trace::TraceLayer::new_for_http());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await?;
        tracing::info!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn handler() -> impl axum::response::IntoResponse {
    HtmlTemplate(HelloTemplate { name: "world".to_string() })
}

#[derive(askama::Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

pub struct HtmlTemplate<T>(T);

impl<T> axum::response::IntoResponse for HtmlTemplate<T>
where
    T: askama::Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to render template: {}", err),
            ).into_response(),
        }
    }
}
