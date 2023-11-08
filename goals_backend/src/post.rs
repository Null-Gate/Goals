use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use surrealdb::opt::PatchOp;

use crate::{
    gen_salt::GenString,
    get_jwt_secret,
    structures::{Claims, DBPost, DBUserInfo, Post, Resp, DB},
};

#[post("/upload_post/{token}")]
pub async fn upload_post(token: Path<String>, post: Json<Post>) -> HttpResponse {
    let db = DB.get().await;

    if db.use_ns("ns").use_db("db").await.is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We are having some problem when opening our database!",
        ));
    }

    match decode::<Claims>(&token, &DecodingKey::from_secret(get_jwt_secret().as_bytes()), &Validation::new(Algorithm::HS256)) {
        Ok(claims) => {
            match db.select::<Option<DBUserInfo>>(("user", &claims.claims.username)).await {
                Ok(Some(_)) => {
                    let post_id = GenString::new().gen_string(10, 20);

                    let post = DBPost {
                        post_id: post_id.clone(),
                        post: post.into_inner(),
                        ..Default::default()
                    };

                    match db.create::<Option<DBPost>>(("post", &post_id)).content(post).await {
                        Ok(Some(s_post)) => {
                            match db.update::<Option<DBUserInfo>>(("user", &claims.claims.username)).patch(PatchOp::add("/up_posts", &format!("post:{post_id}"))).await {
                                Ok(Some(_)) => {
                                    HttpResponse::Ok().json(s_post)
                                },
                                Ok(None) => {
                                    HttpResponse::InternalServerError().json(Resp::new("Sorry We're having some problem in creating you post!"))
                                },
                                Err(_) => {
                                    HttpResponse::InternalServerError().json(Resp::new("Sorry We're having some problem in creating you post!"))
                                }
                            }
                        },
                        Ok(None) => {
                            HttpResponse::InternalServerError().json(Resp::new("Sorry We're having some problem in creating you post!"))
                        },
                        Err(_) => {
                            HttpResponse::InternalServerError().json(Resp::new("Sorry We're having some problem in creating you post!"))
                        }
                    }
                },
                Ok(None) => {
                    HttpResponse::NotAcceptable().json(Resp::new("Sorry User Not Found!"))
                }
                Err(_) => {
                    HttpResponse::InternalServerError().json(Resp::new("Sorry We're having some problem in checking your account!"))
                }
            }
        },
        Err(e) => {
            HttpResponse::NotAcceptable().json(Resp::new(&format!("Sorry Your token is not valid, Please Signup or Login to your account and get a token!{e:?}")))
        }
    }
}

#[get("/default_post")]
pub async fn default_post() -> HttpResponse {
    HttpResponse::Ok().json(Post::default())
}
