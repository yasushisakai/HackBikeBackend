use actix_web::http::StatusCode;
use actix_web::{Error, HttpResponse, Result as ActixResult};
use futures::future::ok as fut_ok;
use futures::Future;

pub fn index() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK).finish())
}

// please reference https://github.com/CityScope/CS_CityIO_Backend for more examples

pub fn get_data() -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().json("get_data"))
}

pub fn set_data() -> impl Future<Item = HttpResponse, Error = Error> {
    fut_ok(HttpResponse::Ok().json("set_data"))
}

