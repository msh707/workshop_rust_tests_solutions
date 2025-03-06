#[cfg(test)]
mod tests{
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
#[tokio::test]
async fn test_response_200() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let status = reqwest::get(format!("{}/hello", &mock_server.uri()))
        .await
        .unwrap()
        .status();
    assert_eq!(status.as_u16(), 200);
}
#[tokio::test]
async fn test_response_404() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let status = reqwest::get(format!("{}/missing", &mock_server.uri()))
        .await
        .unwrap()
        .status();
    assert_eq!(status.as_u16(), 404);
}
#[tokio::test]
async fn test_response_403() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(403))
        .mount(&mock_server)
        .await;
    let status = reqwest::get(format!("{}/hello", &mock_server.uri()))
        .await
        .unwrap()
        .status();
    assert_eq!(status.as_u16(), 403);
}
#[tokio::test]
async fn test_response_401() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;
    let status = reqwest::get(format!("{}/hello", &mock_server.uri()))
        .await
        .unwrap()
        .status();
    assert_eq!(status.as_u16(), 401);
}
#[tokio::test]
async fn test_response_400() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(400))
        .mount(&mock_server)
        .await;
    let status = reqwest::get(format!("{}/hello", &mock_server.uri()))
        .await
        .unwrap()
        .status();
    assert_eq!(status.as_u16(), 400);
}
#[tokio::test]
async fn test_response_201() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;
    let status = reqwest::get(format!("{}/hello", &mock_server.uri()))
        .await
        .unwrap()
        .status();
    assert_eq!(status.as_u16(), 201);
}
}