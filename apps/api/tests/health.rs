use api::create_router;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn health_returns_ok_status_and_expected_body() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body,
        json!({
            "status": "ok",
            "application": "omnitracker"
        })
    );
}

#[tokio::test]
async fn unknown_route_returns_not_found() {
    let response = create_router()
        .oneshot(
            Request::builder()
                .uri("/not-found")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
