use actix_web::{HttpServer, App, get, HttpResponse, Scope, web::{self, Json}};
use surrealdb::{Surreal, engine::local::{File, Db}};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref DB: AsyncOnce<Surreal<Db>> = AsyncOnce::new(async {
        Surreal::new::<File>("db.db").await.unwrap()
    });
}

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
