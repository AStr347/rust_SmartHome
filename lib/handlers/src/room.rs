use std::sync::Mutex;

use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    web::Data,
    {route, HttpRequest, HttpResponse, Responder},
};
use ia_home::{home::Home, room::Room};

///
/// create HttpResponse with plaintext
/// * 'body' - text for response
///
fn bad_req(body: &str) -> HttpResponse<BoxBody> {
    let response = HttpResponse::BadRequest()
        .content_type(ContentType::plaintext())
        .body(body.to_string());

    return response;
}

#[route("/api/smarthome/v1.0/home/rooms/show", method = "GET", method = "POST")]
///
/// GET and POST handler
/// check Room-Name exist and create answer
///
pub async fn show_room(req: HttpRequest, home: Data<Mutex<Home>>) -> impl Responder {
    let headers = req.headers();
    let idres = headers.get("Room-Name");

    if let Some(hid) = idres {
        if let Ok(id) = hid.to_str() {
            let h = home.lock().unwrap();
            let res = h.show_room(id);
            if let Err(e) = &res {
                let body = format!("error: {}", e);
                return bad_req(&body);
            }
            let body = res.unwrap();
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(body);
        }
    }

    return bad_req("show_room called without Room-Name");
}

#[route("/api/smarthome/v1.0/home/rooms/del", method = "GET", method = "POST")]
///
/// GET and POST handler
/// check Room-Name exist and create answer
///
pub async fn del_room(req: HttpRequest, home: Data<Mutex<Home>>) -> impl Responder {
    let headers = req.headers();
    let idres = headers.get("Room-Name");

    if let Some(hid) = idres {
        if let Ok(id) = hid.to_str() {
            let mut h = home.lock().unwrap();
            let res = h.remove_room(id);
            if let Err(e) = res {
                let body = format!("error: {}", e);
                return bad_req(&body);
            }
            let body = format!("room with name {} removed", id);
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(body);
        }
    }

    return bad_req("del_room called without Room-Name");
}

#[route("/api/smarthome/v1.0/home/rooms/add", method = "POST")]
///
/// POST handler
/// check Room-Name exist and create answer
///
pub async fn add_room(req: HttpRequest, home: Data<Mutex<Home>>) -> impl Responder {
    let headers = req.headers();
    let roomres = headers.get("Room-Name");

    if let Some(groom) = roomres {
        if let Ok(room) = groom.to_str() {
            let mut h = home.lock().unwrap();
            let new_room = Room::new(room);
            let res = h.add_room(new_room);
            if let Err(e) = res {
                let body = format!("error: {}", e);
                return bad_req(&body);
            }
            let body = format!("room with name {} added", room);
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(body);
        }
    }

    return bad_req("add_room called without Room-Name");
}
