use std::sync::Mutex;
use super::models::Course;
use sqlx::{postgres::PgPool, FromRow, Row};

//define AppState
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u64>,
    // pub courses: Mutex<Vec<Course>>,
    pub db: sqlx::PgPool,
}


//test
