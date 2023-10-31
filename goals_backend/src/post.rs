use actix_web::{post, HttpResponse, web::{Json, Path}, get};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use surrealdb::opt::PatchOp;

use crate::{structures::{Post, DB, Resp, Claims, DBUserInfo}, get_jwt_secret, gen_salt::GenString};

#[post("/upload_post/{token}")]
pub async fn upload_post(token: Path<String>, post: Json<Post>) -> HttpResponse {
    let db = DB.get().await;
    match decode::<Claims>(&token, &DecodingKey::from_secret(get_jwt_secret().as_bytes()), &Validation::new(Algorithm::HS256)) {
        Ok(claims) => {
            match db.select::<Option<DBUserInfo>>(("user", &claims.claims.username)).await {
                Ok(Some(_)) => {
                    let post_id = GenString::new().gen_string(10, 20);
                    match db.create::<Option<Post>>(("post", &post_id)).content(post).await {
                        Ok(Some(s_post)) => {
                            match db.update::<Option<DBUserInfo>>(("user", &claims.claims.username)).patch(PatchOp::add("/up_posts", &format!("post:{post_id}"))).await {
                                Ok(Some(uinfo)) => {
                                    println!("{uinfo:?}");
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
