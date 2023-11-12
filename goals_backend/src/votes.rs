use actix_web::{get, web::Path, HttpResponse};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use surrealdb::{opt::RecordId, sql::Id};

use crate::{
    get_jwt_secret,
    structures::{Claims, DBPost, DBUserInfo, Resp, DB},
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
                db.select::<Option<DBPost>>(("post", post_id)).await,
            ) {
                (Ok(Some(_)), Ok(Some(mut post))) => {
                    let rid = RecordId::from(("user", Id::String(claims.claims.username.clone())));
                    if let Some(rid) = post.to_owned().up_voters.get(&rid) {
                        post.up_voters.remove(rid);
                    } else if let Some(rid) = post.to_owned().dw_voters.get(&rid) {
                        post.up_voters.insert(rid.to_owned());
                        post.dw_voters.remove(rid);
                    } else {
                        post.up_voters.insert(rid);
                    }
                    post.votes = post.up_voters.len() as isize - post.dw_voters.len() as isize;
                    match db
                        .update::<Option<DBPost>>(("post", post_id))
                        .content(post)
                        .await
                    {
                        Ok(Some(post)) => HttpResponse::Ok().json(post),
                        e => HttpResponse::InternalServerError().json(Resp::new(&format!(
                            "We're having some problem in processing your vote!!{e:?}"
                        ))),
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
                db.select::<Option<DBPost>>(("post", post_id)).await,
            ) {
                (Ok(Some(_)), Ok(Some(mut post))) => {
                    let rid = RecordId::from(("user", Id::String(claims.claims.username.clone())));
                    if let Some(rid) = post.to_owned().up_voters.get(&rid) {
                        post.dw_voters.insert(rid.to_owned());
                        post.up_voters.remove(rid);
                    } else if let Some(rid) = post.to_owned().dw_voters.get(&rid) {
                        post.dw_voters.remove(rid);
                    } else {
                        post.dw_voters.insert(rid.to_owned());
                    }
                    post.votes = post.up_voters.len() as isize - post.dw_voters.len() as isize;

                    match db
                        .update::<Option<DBPost>>(("post", post_id))
                        .content(post)
                        .await
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
