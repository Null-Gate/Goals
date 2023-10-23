use actix_web::{HttpServer, App};
use scopes::auth_scope;

mod secrets;
mod structures;
mod scopes;
mod login;
mod signup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(auth_scope())).bind(("127.0.0.1", 9899))?.run().await
}
