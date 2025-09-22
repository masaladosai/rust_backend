use crate::routes::index::{get_index, post_index};
use crate::routes::login::{login, sign_up};
use axum::{
    routing::{get,post}, Router
};

pub fn route()->Router{
    Router::new()
    .route("/",get(get_index).post(post_index))
    .route("/login",get(login))
    .route("/signup",post(sign_up))
}