use axum::{Json, routing::get};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(OpenApi)]
#[openapi(info(title = "Rust API", version = "0.1.0"))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let port = 3000;
    let host = "localhost";

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(root))
        .split_for_parts();
    let app = router.route(
        "/openapi.json",
        get(move || {
            let api = api.clone();
            async move { Json(api) }
        }),
    );

    let listener = tokio::net::TcpListener::bind(&format!("{}:{}", host, port))
        .await
        .unwrap();

    println!("Listening on http://{}:{}", host, port);

    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/",
    responses((status = 200, description = "Returns a greeting", body = String))
)]
async fn root() -> &'static str {
    "Hello!"
}
