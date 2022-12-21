use actix_web::{web, Responder};
use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
struct TimeResponse {
    status: String,
    timestamp: i64,
    datetime: DateTime<Utc>,
}

#[tracing::instrument(name = "Get current time")]
pub async fn time_handler() -> Result<impl Responder, actix_web::Error> {
    let now = Utc::now();

    let response = TimeResponse {
        status: "Ok".to_string(),
        timestamp: now.timestamp(),
        datetime: now,
    };

    Ok(web::Json(response))
}
