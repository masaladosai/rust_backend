use sea_orm::entity::prelude::*;
use serde::{Deserialize,Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name="userstable")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id:Uuid,
    pub name:String,
    pub email:String
}


#[derive(Copy,Clone,Debug,EnumIter,DeriveRelation)]
pub enum Relation{}


impl ActiveModelBehavior for ActiveModel{}