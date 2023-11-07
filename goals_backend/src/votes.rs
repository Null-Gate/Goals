use actix_web::{post, HttpResponse, web::{Path, self}};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use surrealdb::opt::PatchOp;

use crate::{structures::{DB, Resp, Claims, DBUserInfo, DBPost}, get_jwt_secret};

#[post("up_vote/{post_id}/{token}")]
async fn up_vote(post_id: Path<String>, token: Path<String>) -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
       return HttpResponse::InternalServerError().json(Resp::new("Sorry We are having some problem when opening our database!"));
    }

    match decode::<Claims>(&token, &DecodingKey::from_secret(get_jwt_secret().as_bytes()), &Validation::new(Algorithm::HS256)) {
        Ok(claims) => {
            match (db.select::<Option<DBUserInfo>>(("user", &claims.claims.username)).await, db.select::<Option<DBPost>>(("post", post_id.as_str())).await) {
                (Ok(Some(_)), Ok(Some(post))) => {
                    if post.voters.contains(&format!("user:{}", &claims.claims.username)) {
                        return dw_vote_up_vote(post_id.as_str(), &claims.claims.username, post.votes).await;
                    }
                    match db.update::<Option<DBPost>>(("post", post_id.as_str())).patch(PatchOp::replace("/votes", post.votes + 1)).patch(PatchOp::add("/voters", &format!("user:{}", &claims.claims.username))).await {
                        Ok(Some(post)) => {
                            HttpResponse::Ok().json(post)
                        },
                        _ => {
                            HttpResponse::InternalServerError().json(Resp::new("We're having some problem in processing your vote!!"))
                        },
                    }
                },
                (Ok(Some(_)), Ok(None)) => {
                    HttpResponse::NoContent().json(Resp::new("Sorry The Post you're voting is not found!!"))
                },
                (Ok(None), _) => {
                    HttpResponse::NotAcceptable().json(Resp::new("Are You a Ghost?, We didn't found you're acc!"))
                },
                _ => {
                    HttpResponse::BadRequest().json(Resp::new("Shut The Fuck Off Something went wrong!!"))
                }
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(Resp::new("Sorry your token is not valid or Something went wrong!!"))
        }
    }
}

async fn dw_vote_up_vote(post_id: &str, user_id: &str, votes: usize) -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
       return HttpResponse::InternalServerError().json(Resp::new("Sorry We are having some problem when opening our database!"));
    }
    match db.update::<Option<DBPost>>(("post", post_id)).patch(PatchOp::replace("/votes", votes - 1)).patch(PatchOp::remove(&format!("/voters/user:{user_id}"))).await {
        Ok(Some(post)) => {
            HttpResponse::Ok().json(post)
        },
        _ => {
            HttpResponse::InternalServerError().json(Resp::new("Something went wrong while down voting the post!!"))
        },
    }
}

#[post("dw_vote/{post_id}/{token}")]
async fn dw_vote(post_id: Path<String>, token: Path<String>) -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
       return HttpResponse::InternalServerError().json(Resp::new("Sorry We are having some problem when opening our database!"));
    }

    match decode::<Claims>(&token, &DecodingKey::from_secret(get_jwt_secret().as_bytes()), &Validation::new(Algorithm::HS256)) {
        Ok(claims) => {
            match (db.select::<Option<DBUserInfo>>(("user", &claims.claims.username)).await, db.select::<Option<DBPost>>(("post", post_id.as_str())).await) {
                (Ok(Some(_)), Ok(Some(post))) => {
                    if post.voters.contains(&format!("-user:{}", &claims.claims.username)) {
                        return up_vote_dw_vote(post_id.as_str(), &claims.claims.username, post.votes).await;
                    }
                    match db.update::<Option<DBPost>>(("post", post_id.as_str())).patch(PatchOp::replace("/votes", post.votes - 1)).patch(PatchOp::add("/voters", &format!("-user:{}", &claims.claims.username))).await {
                        Ok(Some(post)) => {
                            HttpResponse::Ok().json(post)
                        },
                        _ => {
                            HttpResponse::InternalServerError().json(Resp::new("We're having some problem in processing your vote!!"))
                        },
                    }
                },
                (Ok(Some(_)), Ok(None)) => {
                    HttpResponse::NoContent().json(Resp::new("Sorry The Post you're voting is not found!!"))
                },
                (Ok(None), _) => {
                    HttpResponse::NotAcceptable().json(Resp::new("Are You a Ghost?, We didn't found you're acc!"))
                },
                _ => {
                    HttpResponse::BadRequest().json(Resp::new("Shut The Fuck Off Something went wrong!!"))
                }
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(Resp::new("Sorry your token is not valid or Something went wrong!!"))
        }
    }
}

async fn up_vote_dw_vote(post_id: &str, user_id: &str, votes: usize) -> HttpResponse {
    todo!()
}

async fn gt_vote() {}
