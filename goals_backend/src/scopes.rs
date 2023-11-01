use actix_web::{web, Scope};

use crate::{
    login::login,
    post::{default_post, upload_post},
    fetch_post::fetch_posts,
    signup::{idk, sign_up},
};
pub fn auth_scope() -> Scope {
    web::scope("/api")
        .service(login)
        .service(fetch_posts)
        .service(upload_post)
        .service(default_post)
        .service(sign_up)
        .service(idk)
}
