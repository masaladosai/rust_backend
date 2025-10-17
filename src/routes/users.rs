use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize,Serialize};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::tables::table::ActiveModel as UserActiveModel;
use crate::tables::table::{Model as UserModel,Entity as UserEntity};

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
    let res=new_user.insert(&db).await.unwrap();
    Json(res)
}


pub async fn del_users(State(db):State<DatabaseConnection>,Path(user_id):Path<Uuid>)->impl IntoResponse{
   let del_result=UserEntity::delete_by_id(user_id).exec(&db).await;

   match del_result{
    Ok(res)=>{
        if res.rows_affected==0{
            (StatusCode::NOT_FOUND, "User not found".to_string())
        }else{
            (StatusCode::NO_CONTENT,"".to_string())
        }
    }
    Err(db_err)=>{
        (StatusCode::INTERNAL_SERVER_ERROR,format!("database error:{}",db_err))

    }
   }

}


pub async fn get_users(
    State(db):State<DatabaseConnection>
)->Result<Json<Vec<UserModel>>,(StatusCode,String)>{
    let all_users=UserEntity::find()
    .all(&db).await.map_err(|db_err|{
        (StatusCode::INTERNAL_SERVER_ERROR,format!("an error occured:{}",db_err))
    })?;


    Ok(Json(all_users))
}