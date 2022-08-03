//active_web
// Language: rust
// Path: webservice/src/bin/server2.rs
//use actix_web
use actix_web::{web, App, HttpServer,HttpResponse,Responder};
use std::io;

//router
pub fn generate_routes(cfg:&mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(heal_check_handler));
}

pub async fn heal_check_handler() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

//define actix_rt::main function
// Language: rust
#[actix_rt::main]
async fn main() -> io::Result<()> {
    //start http server
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/health").to(heal_check_handler))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
