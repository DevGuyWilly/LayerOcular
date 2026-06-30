use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/logout", web::get().to(logout))
    );
}


async fn signup() -> impl Responder { HttpResponse::Ok().body("v1 signup") }
async fn login() -> impl Responder { HttpResponse::Ok().body("v1 login") }
async fn logout() -> impl Responder { HttpResponse::Ok().body("v1 logout") }
