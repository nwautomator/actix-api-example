use actix_api_example::{
    handlers::{home_handler, quote_handler, time_handler},
    telemetry::{get_subscriber, init_subscriber},
};
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("actix-example".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home_handler))
            .route("/time", web::get().to(time_handler))
            .route("/quote", web::get().to(quote_handler))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
