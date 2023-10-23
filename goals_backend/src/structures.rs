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
        Surreal::new::<File>("db.db").await.unwrap()
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

#[derive(Serialize, Deserialize)]
pub struct SignUpInfo {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub upfp_pic: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Time {
    TimeStamp(usize),
    Other(String),
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Date {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub details: String,
    pub tables: HashMap<Date, HashMap<Time, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub up_posts: Vec<Post>,
    pub pic_path: String,
}
