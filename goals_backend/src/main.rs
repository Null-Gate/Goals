use std::collections::HashMap;

use jsonwebtoken;
use actix_web::{HttpServer, App, get, HttpResponse, Scope, web::{self, Json}};
use surrealdb::{Surreal, engine::local::{File, Db}, opt::QueryResult};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref DB: AsyncOnce<Surreal<Db>> = AsyncOnce::new(async {
        Surreal::new::<File>("db.db").await.unwrap()
    });
}

#[derive(Serialize, Deserialize)]
struct Resp {
    msg: String,
}

impl Resp {
    fn new(msg: &str) -> Resp {
        Resp { msg: msg.into() }
    }
}

#[derive(Serialize, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct SignUpInfo {
    username: String,
    fullname: String,
    password: String,
    upfp_pic: Option<Vec<u8>>
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
enum Time {
    TimeStamp (usize),
    Other (String),
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
enum Date {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Serialize, Deserialize)]
struct Post {
    title: String,
    details: String,
    tables: HashMap<Date, HashMap<Time, String>>,
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    username: String,
    fullname: String,
    password: String,
    up_posts: Vec<Post>,
    pic_path: String,
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
    let db = DB.get().await;
    let sql = r#"SELECT username, fullname, password "#;
    match db.query(sql).await {
        Ok(mut user_check) => {
            if let Ok(Some(user)) = user_check.take::<Option<String>>("password") {
            }
            HttpResponse::NotFound().json(Resp::new(&format!("Sorry No User is founded with username of: {}", info.username)))
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(Resp::new("Sorry We are some problem in opening database!!"))
        }
    }
}
