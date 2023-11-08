use actix_web::{web, Scope};

use crate::{
    fetch_post::fetch_posts,
    login::login,
    post::{default_post, upload_post},
    signup::{idk, sign_up},
    votes::{dw_vote, up_vote},
};
pub fn auth_scope() -> Scope {
    web::scope("/api")
        .service(login)
        .service(fetch_posts)
        .service(upload_post)
        .service(default_post)
        .service(sign_up)
        .service(idk)
        .service(up_vote)
        .service(dw_vote)
}
