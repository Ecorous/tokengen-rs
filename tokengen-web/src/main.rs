use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct GenerateResponse {
    length: i32,
    token: String
}

#[get("/generate/{length}")]
async fn generate(path: actix_web::web::Path<i32>) -> impl Responder {
    let length = path.into_inner();
    let token = tokengen_core::generate(Some(length));
    let response = GenerateResponse { 
        length,
        token
    };
    let x = serde_json::to_string(&response).unwrap_or(String::from("{}"));
    HttpResponse::Ok().content_type("application/json").body(x)
}

#[get("/generate")]
async fn generate_nopath() -> impl Responder {
    let token = tokengen_core::generate(None);
    let response = GenerateResponse { 
        length: 32,
        token
    };
    let x = serde_json::to_string(&response).unwrap_or(String::from("{}"));
    HttpResponse::Ok().content_type("application/json").body(x)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .service(generate)
            .service(generate_nopath)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
