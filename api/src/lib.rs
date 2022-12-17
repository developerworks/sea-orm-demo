use actix_web::web;
use sea_orm::{DatabaseConnection, Database};
pub mod patient_controller;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}


fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(patient_controller::view);
}