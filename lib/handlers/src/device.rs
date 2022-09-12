use std::sync::Mutex;

use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    web::Data,
    {route, HttpRequest, HttpResponse, Responder},
};
use ia_devices::DeviceBuilder;
use ia_home::{home::Home, homeerrors::HomeErrors};

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

#[route(
    "/api/smarthome/v1.0/home/rooms/devices/show",
    method = "GET",
    method = "POST"
)]
///
/// GET and POST handler
/// check Room-Name and Device-Name exist, and create answer
///
pub async fn show_device(req: HttpRequest, home: Data<Mutex<Home>>) -> impl Responder {
    let headers = req.headers();
    let roomres = headers.get("Room-Name");
    let deviceres = headers.get("Device-Name");

    if let (Some(rid), Some(did)) = (roomres, deviceres) {
        if let (Ok(room), Ok(device)) = (rid.to_str(), did.to_str()) {
            let h = home.lock().unwrap();
            let res = h.show_device(room, device);
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

    return bad_req("show_room called without Room-Name or Device-Name");
}

#[route(
    "/api/smarthome/v1.0/home/rooms/devices/del",
    method = "GET",
    method = "POST"
)]
///
/// GET and POST handler
/// check Room-Name and Device-Name exist, and create answer
///
pub async fn del_device(req: HttpRequest, home: Data<Mutex<Home>>) -> impl Responder {
    let headers = req.headers();
    let roomres = headers.get("Room-Name");
    let deviceres = headers.get("Device-Name");

    if let (Some(rid), Some(did)) = (roomres, deviceres) {
        if let (Ok(room), Ok(device)) = (rid.to_str(), did.to_str()) {
            let mut h = home.lock().unwrap();
            let res = h.remove_device(room, device);
            if let Err(e) = res {
                let body = format!("error: {}", e);
                return bad_req(&body);
            }
            let body = format!(
                "device with name {} in room witn name {} removed",
                device, room
            );
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(body);
        }
    }

    return bad_req("del_room called without Room-Name or Device-Name");
}

#[route(
    "/api/smarthome/v1.0/home/rooms/devices/add",
    method = "POST"
)]
///
/// POST handler
/// check Room-Name and Device exist, and create answer
///
pub async fn add_device(req: HttpRequest, home: Data<Mutex<Home>>) -> impl Responder {
    let headers = req.headers();
    let roomres = headers.get("Room-Name");
    let deviceres = headers.get("DeviceBuilder");

    if let (Some(rid), Some(dval)) = (roomres, deviceres) {
        if let (Ok(room), Ok(device)) = (rid.to_str(), dval.to_str()) {
            println!("room:{}\t\tdevice:{}", room, device);
            let device_builder_res: Result<DeviceBuilder, serde_json::Error> =
                serde_json::from_str(device);
            if let Err(e) = &device_builder_res {
                let body = format!("parce error: {}", e);
                return bad_req(&body);
            }
            let device_builder = device_builder_res.unwrap();
            let builded = device_builder.build();

            let mut h = home.lock().unwrap();
            let add_res: Result<(), HomeErrors>;
            match builded {
                ia_devices::BuildedDevice::None => {
                    let body = format!("device type is None");
                    return bad_req(&body);
                }
                ia_devices::BuildedDevice::BuildedSoket(soket) => {
                    add_res = h.add_device(room, soket);
                }
                ia_devices::BuildedDevice::BuildedWindow(window) => {
                    add_res = h.add_device(room, window);
                }
                ia_devices::BuildedDevice::BuildedWeather(weather) => {
                    add_res = h.add_device(room, weather);
                }
            }
            if let Err(e) = add_res {
                let body = format!("{}", e);
                return bad_req(&body);
            }

            let body = format!("device: {} in room witn name {} added", device, room);
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(body);
        }
    }

    return bad_req("add_room called without Room-Name or DeviceBuilder");
}
