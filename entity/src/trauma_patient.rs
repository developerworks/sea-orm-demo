//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "trauma_patient"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub name: Option<String>,
    pub admission_number: Option<String>,
    pub emergency_number: Option<String>,
    pub number_of_admission: Option<i32>,
    pub be_in_hospitalization: Option<i8>,
    pub the_green_channel: Option<i8>,
    pub the_soldier: Option<i8>,
    pub be_emergency_case: Option<i8>,
    pub be_transfer_hospital: Option<i8>,
    pub identification_type: Option<i16>,
    pub identification_card: Option<String>,
    pub birthday: Option<String>,
    pub age_unit: Option<i32>,
    pub age: Option<i32>,
    pub gender: Option<i32>,
    pub nation: Option<String>,
    pub occupation: Option<String>,
    pub marriage: Option<String>,
    pub be_admission_time: Option<DateTime>,
    pub be_admission_hospital: Option<String>,
    pub be_admission_department: Option<String>,
    pub region_id: Option<i32>,
    pub nationality: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub address: Option<String>,
    pub telephone: Option<String>,
    pub contact: Option<String>,
    pub contact_telephone: Option<String>,
    pub completion_ratio: Option<f32>,
    pub patient_info_completion_ratio: Option<f32>,
    pub created_at: Option<DateTime>,
    pub created_by: Option<String>,
    pub created_dept: Option<String>,
    pub state: Option<String>,
    pub begin_rescue_time: Option<DateTime>,
    pub dilivery_time: Option<DateTime>,
    pub doctor_delivery_time: Option<DateTime>,
    pub updated_by: Option<String>,
    pub updated_at: Option<DateTime>,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime>,
    pub deleted: Option<i8>,
    pub deleted_at: Option<DateTime>,
    pub deleted_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    AdmissionNumber,
    EmergencyNumber,
    NumberOfAdmission,
    BeInHospitalization,
    TheGreenChannel,
    TheSoldier,
    BeEmergencyCase,
    BeTransferHospital,
    IdentificationType,
    IdentificationCard,
    Birthday,
    AgeUnit,
    Age,
    Gender,
    Nation,
    Occupation,
    Marriage,
    BeAdmissionTime,
    BeAdmissionHospital,
    BeAdmissionDepartment,
    RegionId,
    Nationality,
    Country,
    Province,
    City,
    District,
    Address,
    Telephone,
    Contact,
    ContactTelephone,
    CompletionRatio,
    PatientInfoCompletionRatio,
    CreatedAt,
    CreatedBy,
    CreatedDept,
    State,
    BeginRescueTime,
    DiliveryTime,
    DoctorDeliveryTime,
    UpdatedBy,
    UpdatedAt,
    ApprovedBy,
    ApprovedAt,
    Deleted,
    DeletedAt,
    DeletedBy,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::Name => ColumnType::String(Some(16u32)).def().null(),
            Self::AdmissionNumber => ColumnType::String(Some(32u32)).def().null().unique(),
            Self::EmergencyNumber => ColumnType::String(Some(32u32)).def().null(),
            Self::NumberOfAdmission => ColumnType::Integer.def().null(),
            Self::BeInHospitalization => ColumnType::TinyInteger.def().null(),
            Self::TheGreenChannel => ColumnType::TinyInteger.def().null(),
            Self::TheSoldier => ColumnType::TinyInteger.def().null(),
            Self::BeEmergencyCase => ColumnType::TinyInteger.def().null(),
            Self::BeTransferHospital => ColumnType::TinyInteger.def().null(),
            Self::IdentificationType => ColumnType::SmallInteger.def().null(),
            Self::IdentificationCard => ColumnType::String(Some(18u32)).def().null(),
            Self::Birthday => ColumnType::String(Some(10u32)).def().null(),
            Self::AgeUnit => ColumnType::Integer.def().null(),
            Self::Age => ColumnType::Integer.def().null(),
            Self::Gender => ColumnType::Integer.def().null(),
            Self::Nation => ColumnType::String(Some(255u32)).def().null(),
            Self::Occupation => ColumnType::String(Some(255u32)).def().null(),
            Self::Marriage => ColumnType::String(Some(255u32)).def().null(),
            Self::BeAdmissionTime => ColumnType::DateTime.def().null(),
            Self::BeAdmissionHospital => ColumnType::String(Some(32u32)).def().null(),
            Self::BeAdmissionDepartment => ColumnType::String(Some(16u32)).def().null(),
            Self::RegionId => ColumnType::Integer.def().null(),
            Self::Nationality => ColumnType::String(Some(32u32)).def().null(),
            Self::Country => ColumnType::String(Some(32u32)).def().null(),
            Self::Province => ColumnType::String(Some(255u32)).def().null(),
            Self::City => ColumnType::String(Some(32u32)).def().null(),
            Self::District => ColumnType::String(Some(32u32)).def().null(),
            Self::Address => ColumnType::String(Some(128u32)).def().null(),
            Self::Telephone => ColumnType::String(Some(255u32)).def().null(),
            Self::Contact => ColumnType::String(Some(255u32)).def().null(),
            Self::ContactTelephone => ColumnType::String(Some(32u32)).def().null(),
            Self::CompletionRatio => ColumnType::Float.def().null(),
            Self::PatientInfoCompletionRatio => ColumnType::Float.def().null(),
            Self::CreatedAt => ColumnType::DateTime.def().null(),
            Self::CreatedBy => ColumnType::String(Some(32u32)).def().null(),
            Self::CreatedDept => ColumnType::String(Some(32u32)).def().null(),
            Self::State => ColumnType::String(Some(255u32)).def().null(),
            Self::BeginRescueTime => ColumnType::DateTime.def().null(),
            Self::DiliveryTime => ColumnType::DateTime.def().null(),
            Self::DoctorDeliveryTime => ColumnType::DateTime.def().null(),
            Self::UpdatedBy => ColumnType::String(Some(32u32)).def().null(),
            Self::UpdatedAt => ColumnType::DateTime.def().null(),
            Self::ApprovedBy => ColumnType::String(Some(32u32)).def().null(),
            Self::ApprovedAt => ColumnType::DateTime.def().null(),
            Self::Deleted => ColumnType::TinyInteger.def().null(),
            Self::DeletedAt => ColumnType::DateTime.def().null(),
            Self::DeletedBy => ColumnType::String(Some(255u32)).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
