use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, HttpRequest, HttpResponse};
use tracing::info;

pub async fn default(req: HttpRequest) -> HttpResponse {
    info!(
        "{}: Wrong request to {}: {}",
        req.peer_addr().unwrap().to_string(),
        req.method(),
        req.path()
    );
    HttpResponse::NotFound()
        .content_type("text/HTML")
        .body("<h1>Nothing here!</h1>")
}

pub async fn root(req: HttpRequest) -> HttpResponse {
    info!("{}: Root request", req.peer_addr().unwrap().to_string());
    HttpResponse::Ok()
        .content_type("text/HTML")
        .body("<h1>Nothing here!</h1>")
}

#[allow(clippy::unnecessary_wraps)]
pub fn handle_bad_request<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, actix_web::Error> {
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_default_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = default(req).await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_root_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = root(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
