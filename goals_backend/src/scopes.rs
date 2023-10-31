use actix_web::{web, Scope};

use crate::{
    login::login,
    post::{default_post, upload_post},
    signup::{idk, sign_up},
};
pub fn auth_scope() -> Scope {
    web::scope("/api")
        .service(login)
        .service(upload_post)
        .service(default_post)
        .service(sign_up)
        .service(idk)
}
