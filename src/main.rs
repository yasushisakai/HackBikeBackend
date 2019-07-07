mod handlers;

use std::env;

use actix_web::http::{header};
use actix_web::middleware::{Logger, NormalizePath};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use log::info;

use handlers::{index, get_data, set_data};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde_json::Value;

pub type JSONState = Arc<Mutex<HashMap<String, Value>>>;

fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "actix_web=info,hackbike_backend=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info,hackbike_backend=info");
    }

    env_logger::init();

    let port: String;

    match env::args().nth(1) {
        Some(new_port) => port = new_port,
        None => port = "8080".to_string(),
    }

    let hashmap: JSONState = Arc::new(Mutex::new(HashMap::new()));

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
            .service(web::resource("/api/data").route(web::get().to_async(get_data)))
            .service(web::resource("/api/hoge").route(web::post().to_async(set_data)))
            .service(web::resource("/").route(web::get().to(index)))
        ////////////////////////////////////////////////////////////////////////////////
    })
    .bind(format!("127.0.0.1:{}", &port))
    .and_then(|result| {
        info!("server started, running @ {}", &port);
        Ok(result)
    })?
    .run()
}
