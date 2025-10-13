use axum::{extract::State, Json};
use serde::{Deserialize,Serialize};
use sea_orm::{ActiveModelTrait, DatabaseConnection, ModelTrait, Set};
use uuid::Uuid;

use crate::tables::table::ActiveModel as UserActiveModel;
use crate::tables::table::Model as UserModel;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name:String,
    pub email:String

}



pub async fn create_user(
    State(db):State<DatabaseConnection>,
    Json(payload):Json<CreateUser>
)->Json<UserModel>{
    let new_user=UserActiveModel{
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        email: Set(payload.email),
    };
    let(res)=new_user.insert(&db).await.unwrap();
    Json(res)
}