use std::collections::HashMap;

use argon2::verify_encoded;
use chrono::{Utc, Duration};
use actix_web::{HttpServer, App, get, HttpResponse, Scope, web::{self, Json}};
use jsonwebtoken::{encode, Header, EncodingKey};
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
struct Claims {
    username: String,
    password: String,
    exp: usize,
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
            if let Ok(Some(pass)) = user_check.take::<Option<String>>("password") {
                match verify_encoded(&pass, info.password.as_bytes()) {
                    Ok(status) => {
                        if !status {
                            HttpResponse::InternalServerError().json(Resp::new("Sorry Password didn't matched!"))
                        } else {
                            let exp = (Utc::now() + Duration::days(9999999)).timestamp() as usize;
                            let claims = Claims {
                                username: info.username.clone(),
                                password: info.password.clone(),
                                exp
                            };
                            let jwt = encode(&Header::default(), &claims, &EncodingKey::from_secret(todo!()));
                            todo!()
                        }
                    },
                    Err(_) => HttpResponse::InternalServerError().json(Resp::new("Sorry We are having some problem when checking your password!"))
                }
            } else {
                HttpResponse::NotFound().json(Resp::new(&format!("Sorry No User is founded with username of: {}", info.username)))
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(Resp::new("Sorry We are some problem in opening database!!"))
        }
    }
}
