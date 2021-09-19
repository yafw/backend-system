use actix_web::{HttpResponse};

use std::process::Command;

pub async fn get_devices() -> HttpResponse {

    let json = Command::new("/usr/sbin/ip").arg("-j").arg("link")
        .output()
        .expect("Something");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json.stdout)
}