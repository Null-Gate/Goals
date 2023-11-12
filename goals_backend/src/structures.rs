use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use surrealdb::{
    engine::local::{Db, File},
    opt::RecordId,
    Surreal,
};

use crate::get_cache_dir;

lazy_static! {
    pub static ref DB: AsyncOnce<Surreal<Db>> = AsyncOnce::new(async {
        Surreal::new::<File>(format!("{}/db.db", get_cache_dir().await))
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

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub enum Time {
    TimeStamp((u8, u8)),
    Other(String),
}

impl Default for Time {
    fn default() -> Self {
        Self::TimeStamp((0, 0))
    }
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Default, Debug)]
pub enum Date {
    #[default]
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub title: String,
    pub details: String,
    pub tables: HashMap<Date, HashMap<String, Time>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DBPost {
    pub post_id: String,
    pub post: Post,
    pub votes: isize,
    pub up_voters: HashSet<RecordId>,
    pub dw_voters: HashSet<RecordId>,
}

impl Default for DBPost {
    fn default() -> Self {
        let mut tables = HashMap::new();
        let mut nst_hm = HashMap::new();
        nst_hm.insert(String::default(), Time::default());
        tables.insert(Date::default(), nst_hm);
        Self {
            post_id: String::default(),
            post: Post::default(),
            votes: 0,
            up_voters: HashSet::default(),
            dw_voters: HashSet::default(),
        }
    }
}

impl Default for Post {
    fn default() -> Self {
        let mut tables = HashMap::new();
        let mut nst_hm = HashMap::new();
        nst_hm.insert(String::default(), Time::default());
        tables.insert(Date::default(), nst_hm);
        Self {
            title: String::new(),
            details: String::new(),
            tables,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub up_posts: Vec<Post>,
    pub pic_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DBUserInfo {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub up_posts: Vec<RecordId>,
    pub pic_path: String,
}
