use actix_web::{HttpResponse, Responder};
//use actix_web::{get, post, web, HttpResponse, Responder, Result};
// use log::info;
// use serde::Deserialize;

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

// Experimental code
#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone)]
pub struct MyApp {
    name: String,
}

impl MyApp {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    // Handler with path parameter
    pub async fn greet(&self) -> impl Responder {
        HttpResponse::Ok().body("[my_app] Hello!".to_string())
    }
}
