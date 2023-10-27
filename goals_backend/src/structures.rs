use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};

lazy_static! {
    pub static ref DB: AsyncOnce<Surreal<Db>> = AsyncOnce::new(async {
        Surreal::new::<File>("/home/walker/rust/projects/Goals/goals_backend/db.db")
            .await
            .unwrap()
    });
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub password: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Resp {
    pub msg: String,
}

impl Resp {
    pub fn new(msg: &str) -> Resp {
        Resp { msg: msg.into() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(MultipartForm)]
pub struct SignUpInfo {
    pub username: Text<String>,
    pub fullname: Text<String>,
    pub password: Text<String>,
    pub upfp_pic: TempFile,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum Time {
    TimeStamp(usize),
    Other(String),
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum Date {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub title: String,
    pub details: String,
    pub tables: HashMap<Date, HashMap<Time, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub up_posts: Vec<Post>,
    pub pic_path: String,
}
