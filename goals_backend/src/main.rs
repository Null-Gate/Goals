use actix_web::{HttpServer, App, get, HttpResponse, Scope, web::{self, Json}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct LoginInfo {
    username: String,
    fullname: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(scope())).bind(("127.0.0.1", 9899))?.run().await
}

fn scope() -> Scope {
    web::scope("/api").service(login)
}

#[get("/login")]
async fn login(info: Json<LoginInfo>) -> HttpResponse {
    todo!()
}
