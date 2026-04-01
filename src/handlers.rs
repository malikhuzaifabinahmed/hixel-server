use actix_web::{get, post, web, HttpResponse, Responder};
use tokio::sync::mpsc;
use crate::models::ShiftData;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[post("/echo")]
pub async fn echo(req_body: web::Json<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(req_body.into_inner())
}

#[post("/ingest")]
pub async fn ingest(
    payload: web::Json<ShiftData>, 
    tx: web::Data<mpsc::Sender<ShiftData>>, 
) -> impl Responder {
    let inner_payload = payload.into_inner();
    match tx.send(inner_payload).await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Payload queued for processing"
            }))
        }
        Err(e) => {
            eprintln!("Failed to send payload to channel: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Internal channel closed"
            }))
        }
    }
}
