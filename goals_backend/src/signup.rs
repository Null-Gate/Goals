use actix_web::{post, web::Json, HttpResponse};

use crate::structures::{SignUpInfo, DB};

#[post("/sign_up")]
pub async fn sign_up(info: Json<SignUpInfo>) -> HttpResponse {
    let db = DB.get().await;

    todo!()
}
