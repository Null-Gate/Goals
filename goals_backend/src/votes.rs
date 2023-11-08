use actix_web::{web::Path, HttpResponse, get};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{
    get_jwt_secret,
    structures::{Claims, DBPost, DBUserInfo, Resp, Vote, DB},
};

#[get("up_vote/{post_id}/{token}")]
pub async fn up_vote(paths: Path<(String, String)>) -> HttpResponse {
    let post_id = &paths.0;
    let token = &paths.1;
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We are having some problem when opening our database!",
        ));
    }

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(claims) => {
            match (
                db.select::<Option<DBUserInfo>>(("user", &claims.claims.username))
                    .await,
                db.select::<Option<DBPost>>(("post", post_id))
                    .await,
            ) {
                (Ok(Some(_)), Ok(Some(mut post))) => {
                    if let Some(v) = post
                        .voters
                        .get(&format!("user:{}", &claims.claims.username))
                    {
                        return w_for_vv(&claims.claims.username, v.clone(), &mut post).await;
                    }
                    post.votes += 1;
                    post.voters.insert(format!("user:{}", &claims.claims.username), Vote::Up);
                    match db
                        .update::<Option<DBPost>>(("post", post_id)).content(post).await
                    {
                        Ok(Some(post)) => HttpResponse::Ok().json(post),
                        _ => HttpResponse::InternalServerError().json(Resp::new(
                            "We're having some problem in processing your vote!!",
                        )),
                    }
                }
                (Ok(Some(_)), Ok(None)) => HttpResponse::NoContent()
                    .json(Resp::new("Sorry The Post you're voting is not found!!")),
                (Ok(None), _) => HttpResponse::NotAcceptable()
                    .json(Resp::new("Are You a Ghost?, We didn't found you're acc!")),
                _ => HttpResponse::BadRequest()
                    .json(Resp::new("Shut The Fuck Off Something went wrong!!")),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(Resp::new(
            "Sorry your token is not valid or Something went wrong!!",
        )),
    }
}

async fn w_for_vv(user_id: &str, vote: Vote, post: &mut DBPost) -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We are having some problem when opening our database!",
        ));
    }
    if vote == Vote::Up {
        post.votes -= 1;
    } else {
        post.votes += 1;
    }

    post.voters.remove(&format!("user:{user_id}"));

    post.voters.insert(format!("user:{user_id}"), !vote);
    match db
        .update::<Option<DBPost>>(("post", &post.post_id))
        .content(post)
        .await
    {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        _ => HttpResponse::InternalServerError().json(Resp::new(
            "Something went wrong while removing voting the post!!",
        )),
    }
}

#[get("dw_vote/{post_id}/{token}")]
pub async fn dw_vote(paths: Path<(String, String)>) -> HttpResponse {
    let post_id = &paths.0;
    let token = &paths.1;

    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We are having some problem when opening our database!",
        ));
    }

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(claims) => {
            match (
                db.select::<Option<DBUserInfo>>(("user", &claims.claims.username))
                    .await,
                db.select::<Option<DBPost>>(("post", post_id))
                    .await,
            ) {
                (Ok(Some(_)), Ok(Some(mut post))) => {
                    if let Some(v) = post
                        .voters
                        .get(&format!("user:{}", &claims.claims.username))
                    {
                        return w_for_vv(&claims.claims.username, v.clone(), &mut post).await;
                    }

                    post.votes -= 1;
                    post.voters.insert(format!("user:{}", &claims.claims.username), Vote::Down);

                    match db
                        .update::<Option<DBPost>>(("post", post_id)).content(post).await
                    {
                        Ok(Some(post)) => HttpResponse::Ok().json(post),
                        _ => HttpResponse::InternalServerError().json(Resp::new(
                            "We're having some problem in processing your vote!!",
                        )),
                    }
                }
                (Ok(Some(_)), Ok(None)) => HttpResponse::NoContent()
                    .json(Resp::new("Sorry The Post you're voting is not found!!")),
                (Ok(None), _) => HttpResponse::NotAcceptable()
                    .json(Resp::new("Are You a Ghost?, We didn't found you're acc!")),
                _ => HttpResponse::BadRequest()
                    .json(Resp::new("Shut The Fuck Off Something went wrong!!")),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(Resp::new(
            "Sorry your token is not valid or Something went wrong!!",
        )),
    }
}
