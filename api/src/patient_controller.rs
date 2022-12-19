use actix_web::{HttpResponse, post};
use actix_web::{dev::HttpServiceFactory, get, web, Responder, Result};
use common::ApiResult;
use common::AppState;
use entity::patient;
use tracing::error;

#[get("/{id}")]
async fn view(data: web::Data<AppState>, id: web::Path<u64>) -> Result<impl Responder> {
    let repo = &data.patient_repository;
    let result = repo.get_by_id(id.into_inner()).await;
    // .expect("could not find patient");
    match result {
        Ok(p) => match p {
            Some(patient) => Ok(HttpResponse::Ok().json(ApiResult::new(0, "", web::Json(patient)))),
            None => Ok(HttpResponse::Ok().json(ApiResult::new(0, "Can not find patient", ""))),
        },
        Err(e) => {
            error!("Database error {}", e);
            Ok(HttpResponse::Ok().json(ApiResult::new(0, "Database errors occured", "")))
        }
    }
}

#[allow(unused)]
#[post("/")]
async fn create(data: web::Data<AppState>, patient: web::Json<patient::Model>) -> Result<impl Responder> {
    Ok(web::Json(patient))
}

pub fn api_routes() -> impl HttpServiceFactory {
    web::scope("/patient")
        .service(view)
        .service(create)
}
