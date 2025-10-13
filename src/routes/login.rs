use axum::Json;
use serde::{Deserialize,Serialize};





#[derive(Deserialize,Serialize,Debug)]
pub struct LogData{
    username:String,
    password:String,
}

#[derive(Serialize)]
pub struct RespData{
    message:String,
    username:String
}

pub async fn login(){
    println!("login page");
}

pub async fn sign_up(body:Json<LogData>)->Json<RespData>{

    println!("signup page hit with json");
    return Json(RespData{
        message:"i like u".to_string(),
        username:"yash".to_string()
    })
}



