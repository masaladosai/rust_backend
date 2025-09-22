use tokio::net::TcpListener;
mod route_fn;
use route_fn::route;
mod routes;
#[tokio::main]
async fn main(){

let app=route();







let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener,app).await.unwrap();
}