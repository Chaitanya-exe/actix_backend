
use actix_web::{
    web,
    Responder,
    HttpResponse,
    error
};
use serde::{Deserialize, Serialize};
use futures::StreamExt;

const MAX_SIZE: usize = 262_144;

#[derive(Deserialize, Serialize)]
pub struct User{
    id: String,
    username: String,
}

#[derive(Serialize)]
struct Response{
    message: String
}

pub async fn user_func(mut payload : web::Payload) -> impl Responder{
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk ?;

        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }  
        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<User>(&body).unwrap();
    println!("response sent");
    Ok(HttpResponse::Ok().json(obj))
}

pub async fn another_func(mut payload: web::Payload) -> impl Responder{
    let mut bytes = web::BytesMut::new();

    while let Some(item) = payload.next().await{
        let item = item ?;
        if (bytes.len() + item.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow error"));
        }
        println!("chunk: {:?} ", &item);
        bytes.extend_from_slice(&item);
    }
    Ok(HttpResponse::Ok().finish())
}