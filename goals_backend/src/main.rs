use std::path::Path;

use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{App, HttpServer};
use directories::BaseDirs;
use scopes::auth_scope;
use tokio::fs;

mod fetch_post;
mod fetch;
mod gen_salt;
mod login;
mod post;
mod scopes;
mod signup;
mod structures;
mod votes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dir = format!("{}/user_assets", get_cache_dir().await);
    if !Path::new(&dir).exists() {
        fs::create_dir(&dir).await.unwrap()
    }
    HttpServer::new(move || {
        App::new()
            .app_data(TempFileConfig::default().directory(&dir))
            .service(auth_scope())
    })
    .bind(("127.0.0.1", 9899))?
    .run()
    .await
}

fn get_jwt_secret() -> String {
    dotenvy::var("JWT_SECRET_KEY").unwrap()
}

async fn get_cache_dir() -> String {
    let dir = format!(
        "{}/Goals",
        BaseDirs::new().unwrap().cache_dir().to_string_lossy()
    );
    if !Path::new(&dir).exists() {
        fs::create_dir(&dir).await.unwrap()
    }
    dir
}
