use lombok::AllArgsConstructor;
use repository::mysql::patient_repository_impl::PatientRepository;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct AppState {
    pub patient_repository: PatientRepository,
}

#[derive(Debug, Clone, Serialize, AllArgsConstructor)]
pub struct ApiResult<'a, T> {
    pub code: u16,
    pub message: &'a str,
    pub data: T,
}