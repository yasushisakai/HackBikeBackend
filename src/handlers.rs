use actix_web::http::StatusCode;
use actix_web::{Error, HttpRequest, HttpResponse, Result as ActixResult};
use futures::future::ok as fut_ok;
use futures::Future;

use actix_web::web;
use futures::Stream;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use crate::{MemoryData, DATA_DUMP_PATH, DATABASE_PATH};

pub fn index() -> ActixResult<HttpResponse> {
    // let mut file = File::open("html/hello.html").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body("hello world"))
}

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
    fut_ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(contents),
    )
}

pub fn set_data(
    pl: web::Payload,
    _req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        let mut contents = "Data may saved\n".to_string();
        let data = std::str::from_utf8(&body).unwrap();
        let obj: Value = serde_json::from_str(data).unwrap();

        if let Value::String(_) = obj["app_id"] {
            if let Value::Number(_) = obj["start_ts"] {
                let dirname: &str = &*format!("{}/{}", DATABASE_PATH, obj["app_id"].as_str().unwrap());

                match fs::create_dir_all(dirname) {
                    Err(why) => {
                        println!("! {:?}", why.kind());
                        contents = format!("{:?}", why.kind());
                    }
                    Ok(_) => {
                        let filename = format!(
                            "{}/{}_{}.json",
                            dirname,
                            obj["app_id"].as_str().unwrap(),
                            obj["start_ts"]
                        );
                        println!("{}", filename);
                        let mut f = BufWriter::new(fs::File::create(filename).unwrap());
                        f.write_all(data.as_bytes())
                            .expect("could not write to file");
                    }
                }
            } else {
                contents = "type of start_ts is invalid, should be Number\n".to_string();
            }
        } else {
            contents = "type of app_id is invalid, should be String\n".to_string();
        }

        fut_ok(HttpResponse::Ok().content_type("text/html").body(contents))
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
    match fs::read_dir(DATABASE_PATH) {
        Err(why) => {
            contents = format!("{:?}", why.kind());
            println!("! {:?}", why.kind())
        }
        Ok(paths) => {
            for (_index, path) in paths.enumerate() {
                let path = path.unwrap().path();
                if path.is_dir() {
                    let app_id = path.to_str().unwrap();
                    let app_id = app_id.replace("database/", "");
                    contents = format!(
                        "{}\n<a href=\"./data/{}\">{}</a><br>",
                        contents, app_id, app_id
                    );
                }
            }
        }
    }
    let html = contents_head.to_owned() + &contents + "\n</body>";
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(html))
}

pub fn load_json(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    //println!("{:?}", req);
    let mut contents = String::new();

    match fs::read_dir(format!(
        "{}/{}",
        DATABASE_PATH,
        req.match_info().get("app_id").unwrap()
    )) {
        Err(why) => {
            contents = format!(
                "{:?}: app_id={}",
                why.kind(),
                req.match_info().get("app_id").unwrap()
            );
            println!("! {:?}", why.kind())
        }
        Ok(paths) => {
            for (index, path) in paths.enumerate() {
                let path = path.unwrap().path();
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.unwrap(); // Ignore errors.
                    if index == 0 {
                        contents = line.to_string();
                    } else {
                        contents = format!("{},\n{}", contents, line);
                    }
                }
            }
            contents = "{\"data\":[\n".to_owned() + &contents + "\n]}";
        }
    }

    fut_ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(contents),
    )
}

pub fn get_device(
    device_name: web::Path<String>,
    data: web::Data<MemoryData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let map = data.lock().unwrap();
    let device_name = device_name.to_owned();

    let devices: &Map<String, Value> = match map.get("devices") {
        Some(d) => d.as_object().unwrap(),
        None => return fut_ok(HttpResponse::Ok().body("devices are empty")),
    };

    let device_info: &Value = match devices.get(&device_name) {
        Some(d) => d,
        None => return fut_ok(HttpResponse::Ok().body("cannot find device")),
    };

    fut_ok(HttpResponse::Ok().json(device_info))
}

pub fn get_devices(data: web::Data<MemoryData>) -> impl Future<Item = HttpResponse, Error = Error> {
    let map = data.lock().unwrap();
    let devices: &Value = match map.get("devices") {
        Some(d) => d,
        None => return fut_ok(HttpResponse::Ok().body("devices are empty")),
    };
    fut_ok(HttpResponse::Ok().json(devices))
}

pub fn set_device(
    pl: web::Payload,
    device_name: web::Path<String>,
    data: web::Data<MemoryData>,
    req: web::HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(move |body| {
        let device_name: String = device_name.to_owned();

        let mut map = data.lock().unwrap();
        let mut devices: Map<String, Value> = match map.get("devices") {
            Some(ds) => ds.as_object().unwrap().to_owned(),
            None => Map::new(),
        };

        let mut json_data: Map<String, Value> = match serde_json::from_slice(&body) {
            Ok(d) => d,
            Err(e) => {
                let mes = format!("{:?}", e);
                return fut_ok(HttpResponse::build(StatusCode::BAD_REQUEST).body(mes));
            }
        };

        match req.connection_info().remote() {
            Some(r) => {
                let remote = r.to_string();
                json_data.insert("ip".to_owned(), json!(remote));
            }
            _ => (),
        };

        devices.insert(device_name.to_owned(), json!(json_data));
        map.insert("devices".to_string(), json!(devices));

        fut_ok(HttpResponse::Ok().body("success"))
    })
}

pub fn upload_file(
    pl: web::Payload,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);
    println!(
        "--- \n filename: {} \n---",
        req.match_info().get("file_name").unwrap()
    );
    let dirname = DATA_DUMP_PATH;
    pl.concat2().from_err().and_then(move |body| {
        let (device, file_name) = path.to_owned();

        //check if there is the device folder
        let device_dir = format!("{}/raw/{}", dirname, &device);
        let device_path = Path::new(&device_dir);
        if !device_path.exists() {
            fs::create_dir_all(&device_path).unwrap();
        }

        let filename = format!("{}/raw/{}/{}", dirname, device, file_name);
        
        println!("{}", &filename);
        
        let mut file = fs::File::create(&filename).unwrap();
        file.write_all(&body).expect("could not write to file");

        fut_ok(
            HttpResponse::Ok()
                .content_type("text/html")
                .body("Request done\n".to_string()),
        )
    })
}
