use derive_new::new;
use entity::patient;
use entity::prelude::Patient;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

#[derive(Clone, Debug, new)]
pub struct PatientRepository {
    pub db: DatabaseConnection,
}

// #[async_trait]
impl PatientRepository {
    pub async fn get_by_id(&self, id: u64) -> Result<Option<patient::Model>, DbErr> {
        println!("Get patient by id: {}", id);

        Patient::find_by_id(id).one(&self.db).await
    }

    // async fn save(db: &DbConn, patient: Patient) -> Result<Option<patient::Model>, DbErr> {
    //     Patient::
    // }
}
