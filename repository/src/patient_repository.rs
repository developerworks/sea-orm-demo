use async_trait::async_trait;
use entity::patient;
use sea_orm::DbErr;

#[rustfmt::skip]
#[async_trait]
pub trait IPatientRepository: Clone {
    async fn get_by_id(&self, id: u64) -> Result<Option<patient::Model>, DbErr>;
    // async fn save     (db: &DbConn, patient: Patient) -> Result<Option<patient::Model>, DbErr>;
    // fn find(&self, username: String) -> Result<Option<Vec<Patient>>>;
    // fn exists(exists: Patient) -> bool;

    // fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<Patient>, DbErr>;
}
