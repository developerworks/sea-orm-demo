use actix_web::web;
pub mod patient_controller;

/**
 * 注册服务
 */
#[allow(unused)]
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(patient_controller::api_routes());
}
