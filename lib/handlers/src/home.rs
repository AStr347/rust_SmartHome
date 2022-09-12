use std::sync::Mutex;

use actix_web::{
    http::header::ContentType,
    web::Data,
    {route, HttpResponse, Responder},
};
use ia_home::home::Home;

#[route(
    "/api/smarthome/v1.0",
    method = "GET",
    method = "HEAD"
)]
///
/// GET and HEAD handler
/// always return HttpResponse with "check_health" text
///
pub async fn check_health() -> impl Responder {
    let response = HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("check_health");
    return response;
}

#[route(
    "/api/smarthome/v1.0/home/show",
    method = "GET",
    method = "POST"
)]
///
/// GET and HEAD handler
/// always return HttpResponse with "check_health" text
///
pub async fn show_home(home: Data<Mutex<Home>>) -> impl Responder {
    let h = home.lock().unwrap();
    let body = format!("{}", h);

    println!("{}", body);

    let response = HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(body);
    return response;
}
