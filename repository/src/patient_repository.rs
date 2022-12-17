use anyhow::Result;
use entity::prelude::Patient;
pub trait IPatientRepository: Clone {
    fn get_by_id(&self, id: u64) -> Result<Option<Patient>>;
    // fn save(&self, patient: Patient) -> Result<()>;
    // fn find(&self, username: String) -> Result<Option<Vec<Patient>>>;
    // fn exists(exists: Patient) -> bool;
}
