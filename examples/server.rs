use std::sync::Mutex;
use actix_web::{web::Data, App, HttpServer};
use handlers::*;
use ia_home::home::Home;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let home = Home::new("Actix SmartHome".to_string());

    let actix_data = Data::new(Mutex::new(home));

    HttpServer::new(move || {
        App::new()
            .service(home::check_health)
            .service(home::show_home)
            .service(room::show_room)
            .service(room::del_room)
            .service(room::add_room)
            .service(device::show_device)
            .service(device::del_device)
            .service(device::add_device)
            .app_data(actix_data.clone())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}