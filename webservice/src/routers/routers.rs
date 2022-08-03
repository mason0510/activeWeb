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

// pub fn course_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/courses")
//             .route("/", web::post().to(post_new_course))
//             .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
//             .route(
//                 "/{teacher_id}/{course_id}",
//                 web::get().to(get_course_detail),
//             )
//             .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
//             .route(
//                 "/{teacher_id}/{course_id}",
//                 web::put().to(update_course_details),
//             ),
//     );
    /*
    curl -H "Content-Type: application/json" -X POST -d '{"teacher_id":1, "name":"First course"}' "127.0.0.1:3000/courses/"

    增加了数据库后
    curl -H "Content-Type: application/json" -X POST -d '{"teacher_id":1, "id":7, "name":"Calculus"}' "127.0.0.1:3000/courses/"
    */
// }



pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(heal_check_handler));
}





// pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/teachers")
//             .route("/", web::post().to(post_new_teacher))
//             .route("/", web::get().to(get_all_teachers))
//             .route("/{teacher_id}", web::get().to(get_teacher_details))
//             .route("/{teacher_id}", web::put().to(update_teacher_details))
//             .route("/{teacher_id}", web::delete().to(delete_teacher)),
//     );
// }
