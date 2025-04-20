
use actix_web::{
    web,
    Responder,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User{
    id: String,
    username: String,
}

#[derive(Serialize)]
struct Response{
    message: String
}

pub async fn user_func(data : web::Json<User>) -> impl Responder{

    println!("{} {}", data.id, data.username);
    HttpResponse::Ok().json(Response{
        message: format!("data recieved!!")
    })   
}