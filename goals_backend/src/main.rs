use jsonwebtoken;
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

struct SignUpInfo {
    username: String,
    fullname: String,
    password: String,
    upfp_pic: Option<Vec<u8>>
}

enum Time {
}

enum Date {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

struct Post {
    title: String,
    details: String,
}

struct UserInfo {
    username: String,
    fullname: String,
    password: String,
    up_posts: Vec<>,
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
    match db.select(("user", info.username)).await {
        Ok(user) => {},
        Err(_) => {
            HttpResponse::InternalServerError().json(Resp::new("Sorry We are some problem in opening database!!"))
        }
    }
    

    todo!()
}
