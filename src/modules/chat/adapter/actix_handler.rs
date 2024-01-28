use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{errors::Error, modules::chat::service::Service};

#[derive(Deserialize)]
pub struct ChatRequestBody {
    content: String,
    chat_identifier: String,
}

// Adjust the function signature to match your context, including the data you need to pass
pub async fn chat_handler(
    chat_service: web::Data<Service>,
    req: web::Json<ChatRequestBody>,
) -> Result<HttpResponse, Error> {
    let response = chat_service
        .chat(&req.content, &req.chat_identifier)
        .await?;

    Ok(HttpResponse::Ok().json(response))
}
