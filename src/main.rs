use axum::{
    routing::get, Router
};
use tokio::net::TcpListener;


#[tokio::main]
async fn main(){

let app = Router::new().route("/",get(get_index).post(post_index))
.route("/fun",get(fun));



async fn get_index(){
    println!("get index got triggered");
}

async fn post_index(){
    println!("if this gets triggered then we r cooked");
}

async fn fun(){
    println!("jail to bengalis");
}


let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener,app).await.unwrap();
}