use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
mod route_fn;
use route_fn::route;
mod routes;
mod tables;


#[tokio::main]
async fn main(){

let db_url="postgres://masaladosadocker:postdocker@localhost:5433/mydb";

let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");




let app = route(db);

let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener,app).await.unwrap();
}