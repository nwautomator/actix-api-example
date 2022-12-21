use actix_web::{web, Responder};
use fake::{faker::lorem::en::*, Fake};

#[derive(serde::Serialize)]
struct QuoteResponse {
    status: String,
    quote: String,
}

#[tracing::instrument(name = "Get a quote")]
pub async fn quote_handler() -> Result<impl Responder, actix_web::Error> {
    let quote = Words(10..20).fake::<Vec<String>>().join(" ");

    let response = QuoteResponse {
        status: "Ok".to_string(),
        quote,
    };

    Ok(web::Json(response))
}
