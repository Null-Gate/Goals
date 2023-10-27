use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use actix_multipart::form::MultipartForm;
use actix_web::{get, post, HttpResponse, Responder};
use argon2::{hash_encoded, Config, Variant, Version};

use crate::{
    gen_salt::GenString,
    get_jwt_secret,
    structures::{Claims, Resp, SignUpInfo, UserInfo, DB},
};

#[post("sign_up")]
pub async fn sign_up(MultipartForm(form): MultipartForm<SignUpInfo>) -> HttpResponse {
    let db = DB.get().await;
    db.use_ns("ns").use_db("db").await.unwrap();
    let rand_salt = GenString::new().gen_string(20, 200);
    let arg_cfg = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 655360,
        time_cost: 2,
        lanes: 20,
        hash_length: 50,
        ..Default::default()
    };
    let pic_path = format!(
        "/home/walker/rust/projects/Goals/goals_backend/files/{}-{}",
        GenString::new().gen_string(5, 20),
        form.upfp_pic.file_name.unwrap()
    );
    if form.upfp_pic.file.persist(&pic_path).is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We're having some problem in saving you profile image!",
        ));
    }
    match hash_encoded(form.password.as_bytes(), rand_salt.as_bytes(), &arg_cfg) {
        Ok(hash) => {
            let user_info = UserInfo {
                username: form.username.to_string(),
                fullname: form.fullname.to_string(),
                password: hash,
                up_posts: vec![],
                pic_path,
            };
            match db
                .create::<Option<UserInfo>>(("user", form.username.to_string()))
                .content(user_info.to_owned())
                .await
            {
                Ok(resl) => {
                    if resl.is_none() {
                        return HttpResponse::InternalServerError().json(Resp::new(
                            "Sorry we're having some problem when creating your account! 1",
                        ));
                    }
                    let exp = (Utc::now() + Duration::days(9999999)).timestamp() as usize;
                    let claims = Claims {
                        username: user_info.username,
                        password: user_info.password,
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
                Err(e) => HttpResponse::InternalServerError().json(Resp::new(&format!(
                    "Sorry we're having some problem when creating your account! 2: {:?}",
                    e
                ))),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We're having some problem when encrypting your password!",
        )),
    }
}

#[get("/idk")]
pub async fn idk() -> impl Responder {
    "Bruh".to_owned()
}
