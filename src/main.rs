use actix_web::{get, post, web, App, error , HttpResponse, HttpServer, Responder};
mod handlers;

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
            .service(
                web::scope("/api")
                    .app_data(json_config)
                    .route("/register",web::post().to(handlers::user_func))
            )
                      
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
