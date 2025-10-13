use crate::routes::index::{get_index, post_index};
use crate::routes::login::{login, sign_up};
use crate::routes::users::{create_user};
use axum::{
    routing::{get,post}, Router
};

use sea_orm::{DatabaseConnection};


pub fn route(db:DatabaseConnection)->Router{
    Router::new()
    .route("/",get(get_index).post(post_index))
    .route("/login",get(login))
    .route("/signup",post(sign_up))
    .route("/create_user",post(create_user))
    .with_state(db)
}