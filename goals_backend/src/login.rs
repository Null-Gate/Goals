use actix_web::{post, web::Json, HttpResponse};
use argon2::verify_encoded;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{get_jwt_secret, structures::*};

#[post("/login")]
pub async fn login(info: Json<LoginInfo>) -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We are having some problem when opening our database!",
        ));
    }

    match db
        .select::<Option<DBUserInfo>>(("user", &info.username))
        .await
    {
        Ok(Some(user)) => match verify_encoded(&user.password, info.password.as_bytes()) {
            Ok(status) => {
                if !status {
                    HttpResponse::InternalServerError()
                        .json(Resp::new("Sorry Password didn't matched!"))
                } else {
                    let exp = (Utc::now() + Duration::days(9999999)).timestamp() as usize;
                    let claims = Claims {
                        username: info.username.clone(),
                        password: info.password.clone(),
                        exp,
                    };
                    match encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
                    ) {
                        Ok(token) => HttpResponse::Ok().json(Resp::new(&token)),
                        Err(_) => HttpResponse::InternalServerError().json(Resp::new(
                            "Sorry We are having some problem when make your token!",
                        )),
                    }
                }
            }
            Err(_) => HttpResponse::InternalServerError().json(Resp::new(
                "Sorry We are having some problem when checking your password!",
            )),
        },
        Ok(None) => HttpResponse::NotFound().json(Resp::new(&format!(
            "Sorry No User is founded with username of: {}",
            info.username
        ))),
        Err(e) => HttpResponse::InternalServerError().json(Resp::new(&format!(
            "Sorry We are some problem in opening database!!: {:?}",
            e
        ))),
    }
}
