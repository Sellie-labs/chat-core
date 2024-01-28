use actix_web::web;

use crate::modules::chat::adapter::actix_handler::chat_handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/api")
            .service(web::scope("/chat").route("/session", web::post().to(chat_handler))),
    );
}
