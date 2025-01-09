use axum::Router;
use axum::routing::get;
use newsletter::newsletter_routes;

#[tokio::main]
async fn main() {

    let newsletter_app = newsletter_routes();

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/newsletter", newsletter_app);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}