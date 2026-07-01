use actix_web::{web, HttpResponse, Responder};
use tracing::instrument;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bank")
            .route("/connect", web::get().to(connect_flow))
            .route("/callback", web::get().to(oauth_callback))
    );
}

#[instrument]
async fn connect_flow() -> impl Responder {
    tracing::info!("connect_flow hit");
    HttpResponse::Ok().body("v1 signup")
}
#[instrument]
async fn oauth_callback() -> impl Responder {
    tracing::info!("oauth_callback hit");
    HttpResponse::Ok().body("v1 callback")
}