use actix_web::{web, Scope};

use crate::login::login;
pub fn auth_scope() -> Scope {
    web::scope("/api").service(login)
}
