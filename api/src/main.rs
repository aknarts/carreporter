#[actix_rt::main]
async fn main() -> io::Result<()> {
    let port = "8080".to_string();
    let addr = "127.0.0.1".to_string();
    HttpServer::new(move || {
        let cors = actix_cors::Cors::default().allow_any_origin().allowed_methods(vec![
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::PATCH,
            http::Method::DELETE,
        ]).allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ]);
        App::new().app_data(Data::new(db.clone())).app_data(Data::new(mailer.clone())).wrap(middleware::NormalizePath::trim()).wrap(cors).wrap(
            ErrorHandlers::new().handler(http::StatusCode::BAD_REQUEST, handlers::handle_bad_request),
        ).default_service(web::get().to(handlers::default)).service(web::scope("/").route("", web::get().to(handlers::root)))
    }).bind(format!("{addr}:{port}"))?.run().await
}