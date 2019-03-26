pub mod examples;

use actix_web::http::Method;
use actix_web::{server, App};
use crate::examples::lifetimes::lifetime_endpoint_entry;
use crate::examples::errors::errors_endpoint_entry;

fn main() {
    server::new(move || {
        App::new()
            .resource("/error/{num}", |r| r.method(Method::GET).with(errors_endpoint_entry))
            .resource("/lifetimes", |r| r.method(Method::GET).with(lifetime_endpoint_entry))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();

    println!("Started http server: 127.0.0.1:8088");
}
