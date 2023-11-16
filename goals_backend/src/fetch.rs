use actix_files::NamedFile;
use actix_web::{web::Path, HttpResponse, get};
use surrealdb::sql::Id;

use crate::{structures::{DB, Resp, DBUserInfo, UserInfo, DBPost}, get_cache_dir};

#[get("/user/{username}")]
pub async fn fetch_user(username: Path<String>) -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We are having some problem when opening our database!",
        ));
    }

    match db.select::<Option<DBUserInfo>>(("user", Id::String(username.into_inner()))).await {
        Ok(Some(user)) => {
            let mut up_posts: Vec<DBPost> = vec![];
            for i in user.up_posts {
                match db.select::<Option<DBPost>>(i).await {
                    Ok(Some(dpost)) => {
                        up_posts.push(dpost);
                    }
                    Ok(None) => {break;}
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(Resp::new("Sorry We're having Some problem in getting user's post!"));
                    }
                }
            }
            let guser = UserInfo {
                username: user.username,
                fullname: user.fullname,
                pic_path: user.pic_path,
                up_posts,
            };
            HttpResponse::Ok().json(guser)
        },
        Ok(None) => {
            HttpResponse::NotFound().json(Resp::new("Sorry User Not Found!"))
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(Resp::new("Sorry We're having some problem in getting user's info!"))
        }
    }
}

#[get("/post/{post_id}")]
pub async fn fetch_post(post_id: Path<String>) -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We are having some problem when opening our database!",
        ));
    }

    match db.select::<Option<DBPost>>(("post", Id::String(post_id.into_inner()))).await {
        Ok(Some(post)) => {
            HttpResponse::Ok().json(post)
        },
        Ok(None) => {HttpResponse::NotFound().json(Resp::new("Sorry The Post You're Finding is not found!"))},
        Err(_) => {HttpResponse::InternalServerError().json(Resp::new("Sorry Something Went Wrong We're getting post!"))},
    }
}

#[get("user_pic/{pic_name}")]
pub async fn get_pic(pic_name: Path<String>) -> NamedFile {
    let path = format!("{}/user_assets/{}", get_cache_dir().await, pic_name.into_inner());
    NamedFile::open(path).unwrap()
}
