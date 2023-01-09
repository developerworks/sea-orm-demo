use actix_web::{dev::HttpServiceFactory, get, web, Responder, Result};
use actix_web::{post, HttpResponse};
use common::ApiResult;
use common::AppState;
use entity::patient;
use tracing::error;

/**
 * 通过ID查询患者对象
 */
#[get("/{id}")]
async fn view(state: web::Data<AppState>, id: web::Path<u64>) -> Result<impl Responder> {
    let repo = &state.patient_repository;
    let result = repo.get_by_id(id.into_inner()).await;
    // .expect("could not find patient");
    match result {
        Ok(p) => match p {
            Some(patient) => Ok(HttpResponse::Ok().json(ApiResult::ok(web::Json(patient)))),
            None => Ok(
                HttpResponse::Ok().json(ApiResult::<patient::Model>::err(u16::MAX, "Not found"))
            ),
        },
        Err(e) => {
            error!("Database error {}", e);
            Ok(HttpResponse::Ok().json(ApiResult::<patient::Model>::err(
                u16::MAX,
                "Database errors occured",
            )))
        }
    }
}

#[allow(unused)]
#[post("/")]
async fn create(
    data: web::Data<AppState>,
    patient: web::Json<patient::Model>,
) -> Result<impl Responder> {
    Ok(web::Json(patient))
}

pub fn api_routes() -> impl HttpServiceFactory {
    web::scope("/patient").service(view).service(create)
}
