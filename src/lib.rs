extern crate actix_web;
extern crate serde;

extern crate actix_proc_macros;
pub use actix_proc_macros::*;

pub enum Response<T: serde::Serialize> {
	Json(T),
	Text(String),
	Builder(actix_web::dev::HttpResponseBuilder)
}

#[macro_export]
macro_rules! code {
    ($code: ident) => { ::actix_helper_macros::Response::Builder(::actix_web::HttpResponse::$code()) }
}

