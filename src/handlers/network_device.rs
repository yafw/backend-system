use actix_web::{Error, HttpResponse};

pub async fn get_devices() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}