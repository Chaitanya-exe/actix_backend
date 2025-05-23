use actix_web::{get, post, web, App, error , HttpResponse, HttpServer, Responder, guard};
use handlers::{another_func, user_func};
mod handlers;
mod middleware;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}
#[get("/")]
async fn manual_hello()-> impl Responder{
    HttpResponse::Ok().body("Hello.. There")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting web server");
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req|{
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        
        App::new()
            .app_data(json_config)
            .service(
                web::resource("/api").route(
                        web::route()
                            .guard(guard::Post())
                            .guard(guard::Header("content-type", "application/json"))
                            .to(user_func)
                    )
            )
            .service(
                web::resource("/test").route(
                    web::route().to(another_func)
                )
            )
                      
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
