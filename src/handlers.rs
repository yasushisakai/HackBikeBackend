use actix_web::http::StatusCode;
use actix_web::{Error, HttpResponse, Result as ActixResult};
use futures::future::ok as fut_ok;
use futures::Future;

use futures::Stream;
use actix_web::web;
use std::io::prelude::*;
use std::fs::File;

use std::collections::BTreeMap;

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
    // define JSON item in str
    let data = r#"{"Name":"sensuikan1983", "Age":"110"}"#;
    // convert str to JSON
    let obj: BTreeMap<String, String> = serde_json::from_str(data).unwrap();
    let objs = [obj.clone(), obj.clone()];
    println!("{}, {:?}", data, obj);

    // make body JSON contents with ROW JSON data
    let mut contents = String::new();
    contents = format!("{:?},\n{:?}", objs[0], objs[1]);
    contents = "{\"data\":[\n".to_owned() + &contents + "\n]}";

//    fut_ok(HttpResponse::Ok().json(objs))
    fut_ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(contents)
                    )

}

pub fn set_data(
    pl: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        let data = std::str::from_utf8(&body).unwrap();
        let obj: BTreeMap<String, String> = serde_json::from_str(data).unwrap();
        println!("{}, {:?}", data, obj);
        fut_ok(HttpResponse::Ok().json(obj))
    })
}

