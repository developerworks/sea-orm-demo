use actix_web::{get, web, Error, HttpResponse};
use common::AppState;

#[get("/{id}")]
async fn view(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body(""))
}
