use actix_web::{web, Scope};

use crate::{
    login::login,
    signup::{idk, sign_up},
};
pub fn auth_scope() -> Scope {
    web::scope("/api")
        .service(login)
        .service(sign_up)
        .service(idk)
}
