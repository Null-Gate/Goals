use actix_web::{get, web::Json, HttpResponse};
use argon2::verify_encoded;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{structures::*, secrets::JWT_SECRETS};

#[get("/login")]
pub async fn login(info: Json<LoginInfo>) -> HttpResponse {
    let db = DB.get().await;
    let sql = r#"SELECT password FROM user:$id"#;
    match db.query(sql).bind(("id", &info.username)).await {
        Ok(mut user_check) => {
            if let Ok(Some(pass)) = user_check.take::<Option<String>>("password") {
                match verify_encoded(&pass, info.password.as_bytes()) {
                    Ok(status) => {
                        if !status {
                            HttpResponse::InternalServerError().json(Resp::new("Sorry Password didn't matched!"))
                        } else {
                            let exp = (Utc::now() + Duration::days(9999999)).timestamp() as usize;
                            let claims = Claims {
                                username: info.username.clone(),
                                password: info.password.clone(),
                                exp
                            };
                            match encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRETS.as_bytes())) {
                                Ok(token) => {
                                    HttpResponse::Ok().json(Resp::new(&token))
                                },
                                Err(_) => HttpResponse::InternalServerError().json(Resp::new("Sorry We are having some problem when make your token!"))
                            }
                        }
                    },
                    Err(_) => HttpResponse::InternalServerError().json(Resp::new("Sorry We are having some problem when checking your password!"))
                }
            } else {
                HttpResponse::NotFound().json(Resp::new(&format!("Sorry No User is founded with username of: {}", info.username)))
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(Resp::new("Sorry We are some problem in opening database!!"))
        }
    }
}
