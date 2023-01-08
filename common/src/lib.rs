use lombok::{AllArgsConstructor, Builder};
use repository::mysql::patient_repository_impl::PatientRepository;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct AppState {
    pub patient_repository: PatientRepository,
}

#[derive(Debug, Clone, Serialize, AllArgsConstructor, Default)]
pub struct ApiResult<'a, T> {
    pub code: u16,
    pub message: &'a str,
    pub data: Option<T>,
}

impl<T> ApiResult<'_, T> {
    pub fn ok(data: T) -> ApiResult<'static, T> {
        ApiResult::new(0, "", Some(data))
    }

    pub fn err(code: u16, message: &str) -> ApiResult<'_, T> {
        ApiResult::new(code, message, None)
    }
}
