use actix_web::{web, App, Error, FromRequest, HttpRequest, HttpResponse, HttpServer};use actix_web::error::{ ErrorUnauthorized};
use actix_web::dev::Payload;

fn index(_: Authorized) -> HttpResponse {
    HttpResponse::Ok().body("authorized")
}

pub(crate) struct Authorized;

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Result<Self, Error>;
   // type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if is_authorized(req) {
            Ok(Authorized)
        } else {
            Err(ErrorUnauthorized("not authorized"))?
        }
    }
}

fn is_authorized(req: &HttpRequest) -> bool {
    if let Some(value) = req.headers().get("authorized") {
        // actual implementation that checks header here
        dbg!(value);
        true
    } else {
        false
    }
}
