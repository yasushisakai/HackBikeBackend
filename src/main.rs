mod handlers;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use log::info;

use serde_json::Value;

pub type MemoryData = Arc<Mutex<HashMap<String, Value>>>;

use handlers::{
    get_data, get_device, get_devices, index, list_appid, load_json, set_data, set_device,
    upload_file,
};

fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "actix_web=info,hackbike_backend=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info,hackbike_backend=info");
    }

    env_logger::init();

    let port: String;

    let hashmap: MemoryData = Arc::new(Mutex::new(HashMap::new()));

    match env::args().nth(1) {
        Some(new_port) => port = new_port,
        None => port = "8080".to_string(),
    }

    HttpServer::new(move || {
        App::new()
            .data(hashmap.clone())
            .wrap(Logger::default())
            .wrap(NormalizePath)
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST"])
                    .send_wildcard()
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ]),
            )
            ////////////////////////////////////////////////////////////////////////////////
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/api/data")
                    .route(web::get().to_async(list_appid))
                    .route(web::post().to_async(set_data)),
            )
            .service(web::resource("/api/data/{app_id}").route(web::get().to_async(load_json)))
            .service(
                web::resource("/api/file/{device_id}/{file_name}").route(web::post().to_async(upload_file)),
            )
            .service(web::resource("/test").route(web::get().to_async(get_data)))
            .service(
                web::resource("/api/device/{device_id}")
                    .route(web::get().to_async(get_device))
                    .route(web::post().to_async(set_device)),
            )
            .service(web::resource("/api/devices").route(web::get().to_async(get_devices)))
        ////////////////////////////////////////////////////////////////////////////////
    })
    .bind(format!("127.0.0.1:{}", &port))
    .and_then(|result| {
        info!("server started, running @ {}", &port);
        Ok(result)
    })?
    .run()
}
