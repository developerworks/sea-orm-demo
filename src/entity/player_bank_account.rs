//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "player_bank_account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub player_id: i64,
    pub created_at: DateTime,
    pub realname: String,
    pub bank_name: String,
    pub bank_number: String,
    pub deposit_bank_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
