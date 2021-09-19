use actix_web::web;

use crate::handlers::network_device;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/network-device")
            .service(
                web::resource("")
                    .route(web::get().to(network_device::get_devices))
            )
    );
}