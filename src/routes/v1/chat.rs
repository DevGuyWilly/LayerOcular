use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chats")
            .route("", web::post().to(start_new_chat))
            .route("", web::get().to(list_chats))
            .route("/{chat_id}/", web::post().to(send_message))
            .route("/{chat_id}/", web::get().to(get_chat_history)),
    );
}

async fn start_new_chat() -> impl Responder { HttpResponse::Ok().body("New session started") }
async fn list_chats() -> impl Responder { HttpResponse::Ok().body("List of past sessions") }
async fn send_message(path: web::Path<String>) -> impl Responder { HttpResponse::Ok().body(format!("Message added to {}", path)) }
async fn get_chat_history(path: web::Path<String>) -> impl Responder { HttpResponse::Ok().body(format!("History for {}", path)) }