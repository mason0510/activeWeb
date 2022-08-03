use actix_web::{web, App, HttpServer, middleware, error, HttpResponse, HttpRequest, FromRequest};
use std::{env, io};
use std::future::IntoFuture;
use std::sync::Mutex;
use sqlx::postgres::PgPoolOptions;
use actix_web::dev::{Payload, Service, ServiceResponse};
use actix_web::{dev::Service as _};
use actix_web::error::{Error, ErrorUnauthorized};
use actix_web::http::header::HeaderValue;
use futures::{FutureExt, TryFutureExt};

#[path = "../handlers/handler1.rs"]
mod handler;

#[path = "../state.rs"]
mod state;

#[path = "../routers/routers.rs"]
mod routers;

#[path = "../models/models.rs"]
mod models;

//error.rs
#[path = "../errors.rs"]
mod errors;

#[path = "../middleware/auth.rs"]
mod auth;


use routers::*;
use state::AppState;
use auth::Auth;

#[actix_rt::main]
async fn main() -> io::Result<()>{
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);
    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await
        .unwrap();

    let shared_data=web::Data::new(AppState {
        health_check_response: "OK".to_string(),
        visit_count: Mutex::new(0),
        db: pool,
    });

    // let app=move || App::new().route("/", web::to(Auth));

    let app = move || {
        App::new()
            .wrap(Auth)
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                errors::MyError::InvalidTnput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            // .configure(teacher_routes)
    };

    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}
