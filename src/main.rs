pub mod examples;

use actix_web::http::Method;
use crate::examples::lifetimes::lifetime_endpoint_entry;
use crate::examples::errors::errors_endpoint_entry;
use actix_web::{HttpServer, App, web};

//fn main() {
//    HttpServer::new(move || {
//        App::new()
//            .service(
//                web::resource("/error/{num}")
//                    .route(web::get().to_async(errors_endpoint_entry)))
//            .service(
//                web::resource("/lifetimes")
//                    .route(web::post().to_async(lifetime_endpoint_entry)))
//            .service(
//                web::resource("/futures")
//                    .route(web::post().to_async(lifetime_endpoint_entry)))
//    })
//        .bind(format!("{}:8088", bind))?
//        .start();
//}
