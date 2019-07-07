use actix_web::http::StatusCode;
use actix_web::{Error, HttpResponse, Result as ActixResult};
use futures::future::ok as fut_ok;
use futures::Future;

use futures::Stream;
use actix_web::web;
use std::io::prelude::*;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde_json::Value;

pub type JSONState = Arc<Mutex<HashMap<String, Value>>>;

pub fn index() -> ActixResult<HttpResponse> {
    let mut file     = File::open("html/hello.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
                    .content_type("text/html")
                    .body(contents))
}

// please reference https://github.com/CityScope/CS_CityIO_Backend for more examples

pub fn get_data() -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().json("get_data"))
}

pub fn set_data(
    state: web::Data<JSONState>,
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        println!("{:?}, {:?}", state, body);
        fut_ok(HttpResponse::Ok().json("set_data"))
    })
}

