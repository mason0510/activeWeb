use actix_web::{web};
use crate::handler::{add_course_handler, get_course_detail, get_course_handler, heal_check_handler};

//router
pub fn generate_routes(cfg:&mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(heal_check_handler));
}

// 增加路径 /courses的路由
pub fn course_routes(cfg:&mut web::ServiceConfig) {
    cfg
        .service(web::scope("/courses"))
        .route("/", web::post().to(add_course_handler))
        .route("/{user_id}", web::get().to(get_course_handler))
        .route("/{user_id}/{course_id}", web::get().to(get_course_detail));
}

