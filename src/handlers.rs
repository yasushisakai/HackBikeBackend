use actix_web::http::StatusCode;
use actix_web::{Error, HttpResponse, HttpRequest, Result as ActixResult};
use futures::future::ok as fut_ok;
use futures::Future;

use futures::Stream;
use actix_web::web;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
//use std::fs::OpenOptions;
use std::io::{BufWriter, Write, BufRead, BufReader};

use std::collections::BTreeMap;
use serde_json::Value;

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
    let mut contents = format!("{:?},\n{:?}", objs[0], objs[1]);
    contents = "{\"data\":[\n".to_owned() + &contents + "\n]}";

//    fut_ok(HttpResponse::Ok().json(objs))
    fut_ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(contents)
                    )

}

pub fn set_data(
    pl: web::Payload,
    _req: HttpRequest
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        let data       = std::str::from_utf8(&body).unwrap();
        let obj: Value = serde_json::from_str(data).unwrap();
//        println!("{}, \n{:?}", data, obj);

        if let Value::String(_) = obj["app_id"] {
            if let Value::Number(_) = obj["start_ts"] {
                let dirname: &str = &*format!("database/{}", obj["app_id"].as_str().unwrap());

                match fs::create_dir_all(dirname){
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(_)    => {
                                let filename = format!("{}/{}_{}.json", dirname, obj["app_id"].as_str().unwrap(), obj["start_ts"]);
                                println!("{}", filename);
                                let mut f = BufWriter::new(fs::File::create(filename).unwrap());
                                f.write(data.as_bytes()).unwrap();
                    },
                }
            }
        }

        fut_ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Data may saved\n")))
    })
}

pub fn list_appid() -> ActixResult<HttpResponse> {
    let contents_head = r#"<!DOCTYPE html>
                        <html lang="en">
                        <head>
                            <meta charset="utf-8">
                            <title>APP IDs</title>
                        </head>
                        <body>"#;
    let mut contents = String::new();
    match fs::read_dir("database") {
        Err(why)  => {
            contents = format!("{:?}", why.kind());
            println!("! {:?}", why.kind())
        },
        Ok(paths) => {
            for (_index, path) in paths.enumerate() {
                let path   = path.unwrap().path();
                if path.is_dir(){
                    let app_id = path.to_str().unwrap();
                    let app_id = app_id.replace("database/", "");
                    contents = format!("{}\n<a href=\"./data/{}\">{}</a><br>", contents, app_id, app_id);
                }
            }
        },
    }
    let html = contents_head.to_owned() + &contents + "\n</body>";
    Ok(HttpResponse::build(StatusCode::OK)
                    .content_type("text/html")
                    .body(html))
}

pub fn load_json(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    //println!("{:?}", req);
    let mut contents = String::new();

    match fs::read_dir(format!("database/{}", req.match_info().get("app_id").unwrap())) {
        Err(why)  => {
            contents = format!("{:?}: app_id={}", why.kind(), req.match_info().get("app_id").unwrap());
            println!("! {:?}", why.kind())
        },
        Ok(paths) => {
            for (index, path) in paths.enumerate() {
                let path   = path.unwrap().path();
                let file   = File::open(path).unwrap();
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.unwrap(); // Ignore errors.
                    if index == 0 {
                        contents = format!("{}", line);
                    } else {
                        contents = format!("{},\n{}", contents, line);
                    }
                }
            }
            contents = "{\"data\":[\n".to_owned() + &contents + "\n]}";
        },
    }

    fut_ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(contents)
    )
}
