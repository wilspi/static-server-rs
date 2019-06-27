extern crate actix_web;
use actix_web::{server, App, http::Method, HttpRequest, Responder};
use std::fs;

pub const DEFAULT_PAGE : &'static str = "status";
pub const TEMPLATES_LOCATION : &'static str = "templates/";


fn load_template(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("page_id").unwrap_or(DEFAULT_PAGE);
    let data = fs::read_to_string(format!("{}{}.tmpl", TEMPLATES_LOCATION, to)).expect("Unable to read file");
    data
}

fn main() {
    server::new(|| {
        App::new()
            // .resource("/", |r| r.f(greet))
            .resource("/{page_id}", |r| r.method(Method::GET).f(load_template))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
