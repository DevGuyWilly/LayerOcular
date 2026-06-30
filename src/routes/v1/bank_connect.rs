use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bank")
            .route("/connect", web::get().to(connect_flow))
            .route("/callback", web::get().to(oauth_callback))
    );
}

async fn connect_flow() -> impl Responder { HttpResponse::Ok().body("v1 signup") }
async fn oauth_callback() -> impl Responder { HttpResponse::Ok().body("v1 callback") }