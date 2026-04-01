mod models;
mod handlers;
mod worker;

use actix_web::{web, App, HttpServer};
use rustls_acme::{caches::DirCache, AcmeConfig};
use tokio_stream::StreamExt;
use std::env;
use tokio::sync::mpsc;

use models::ShiftData;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let is_production = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()) == "production";
    let domains = ["huzaifabinahmed.com", "www.huzaifabinahmed.com"];
    let email = "huzaifamalik3216@gmail.com";
    
    let (tx, rx) = mpsc::channel::<ShiftData>(10000);

    let app_state = web::Data::new(tx);

    tokio::spawn(worker::start_background_worker(rx));

    if is_production {
        let mut state = AcmeConfig::new(domains)
            .contact_push(format!("mailto:{}", email))
            .cache(DirCache::new("./.rustls_acme_cache"))
            .directory_lets_encrypt(false)
            .state();

        let default_config = state.default_rustls_config();
        let mut rustls_config = (*default_config).clone();

        rustls_config.alpn_protocols = vec![
            b"h2".to_vec(),
            b"http/1.1".to_vec(),
            b"acme-tls/1".to_vec(),
        ];
        
        tokio::spawn(async move {
            loop {
                match state.next().await {
                    Some(Ok(event)) => println!("ACME event: {:?}", event),
                    Some(Err(e)) => eprintln!("ACME error: {:?}", e),
                    None => break,
                }
            }
        });

        println!("Server running on production (HTTPS)");

        HttpServer::new(move || {
            App::new()
                .app_data(app_state.clone())
                .service(handlers::hello)
                .service(handlers::echo)
                .service(handlers::ingest)
        })
        .bind_rustls_0_23(("0.0.0.0", 443), rustls_config)?
        .run()
        .await
    } else {
        println!("Server running on localhost (HTTP)");

        HttpServer::new(move || {
            App::new()
                .app_data(app_state.clone())
                .service(handlers::hello)
                .service(handlers::echo)
                .service(handlers::ingest)
        })
        .bind(("0.0.0.0", 80))?
        .run()
        .await
    }
}
