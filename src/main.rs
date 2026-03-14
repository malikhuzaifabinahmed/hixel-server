use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[post("/echo")]
async fn echo(req_body: web::Json<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(req_body.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:80");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
