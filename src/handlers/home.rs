use actix_web::{http::header::ContentType, HttpResponse};

#[tracing::instrument(name = "Home page")]
pub async fn home_handler() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta http-equiv="content-type" content="text/html; charset=utf-8">
<title>Aloha!</title>
</head>
<body>
    <p>Greetings!</p>
</body>
</html>
"#,
    ))
}
