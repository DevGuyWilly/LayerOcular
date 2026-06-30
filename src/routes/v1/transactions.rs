use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transactions")
            .route("/ingest", web::post().to(ingest_transactions))
            .route("", web::get().to(get_transactions))
            .route("/count", web::get().to(get_transaction_count))
    );
}


async fn ingest_transactions() -> impl Responder { HttpResponse::Ok().body("Ingest") }
async fn get_transactions() -> impl Responder { HttpResponse::Ok().body("Get all") }
async fn get_transaction_count() -> impl Responder { HttpResponse::Ok().body("Count") }
