use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rustls_acme::{caches::DirCache, AcmeConfig};
use tokio_stream::StreamExt;

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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let domain = "huzaifabinahmed.com";
    let email = "huzaifamalik3216@gmail.com";

    // 1. Configure ACME
    let mut state = AcmeConfig::new([domain])
        .contact_push(format!("mailto:{}", email))
        .cache(DirCache::new("./.rustls_acme_cache"))
        .directory_lets_encrypt(true) // Use .directory_lets_encrypt_staging(true) for dev
        .state();

    // 2. Drive the ACME state in the background
    let rustls_config = state.default_rustls_config();
    tokio::spawn(async move {
        loop {
            match state.next().await {
                Some(Ok(event)) => println!("ACME event: {:?}", event),
                Some(Err(e)) => eprintln!("ACME error: {:?}", e),
                None => break,
            }
        }
    });

    println!("Server running on https://{}", domain);

    // 3. Start the HTTPS server
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind_rustls_0_23(("0.0.0.0", 443), (*rustls_config).clone())?
    .run()
    .await
}
