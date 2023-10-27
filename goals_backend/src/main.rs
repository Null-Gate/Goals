use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{App, HttpServer};
use scopes::auth_scope;

mod gen_salt;
mod login;
mod scopes;
mod signup;
mod structures;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(
                TempFileConfig::default()
                    .directory("/home/walker/rust/projects/Goals/goals_backend/files"),
            )
            .service(auth_scope())
    })
    .bind(("127.0.0.1", 9899))?
    .run()
    .await
}

fn get_jwt_secret() -> String {
    dotenvy::var("JWT_SECRET_KEY").unwrap()
}
