use crate::routes::index::{get_index, post_index};
use crate::routes::login::{login, sign_up};
use crate::routes::users::{create_user,get_users,del_users};
use axum::routing::delete;
use axum::{
    routing::{get,post}, Router
};

use sea_orm::{DatabaseConnection};


pub fn route(db:DatabaseConnection)->Router{
    Router::new()
    .route("/",get(get_index).post(post_index))
    .route("/login",get(login))
    .route("/signup",post(sign_up))
    .route("/users",post(create_user))
    .route("/users/get_users",get(get_users)).route("/users/{user_id}",delete(del_users)).with_state(db)
}