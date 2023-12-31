use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};

use actix_multipart::form::MultipartForm;
use actix_web::{get, post, HttpResponse, Responder};
use argon2::{hash_encoded, Config, Variant, Version};

use crate::{
    gen_salt::GenString,
    get_jwt_secret,
    structures::{Claims, Resp, SignUpInfo, DBUserInfo, DB},
};

#[post("sign_up")]
pub async fn sign_up(MultipartForm(form): MultipartForm<SignUpInfo>) -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
       return HttpResponse::InternalServerError().json(Resp::new("Sorry We are having some problem when opening our database!"));
    }

    let rand_salt = GenString::new().gen_string(20, 200);
    let arg_cfg = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 655360,
        time_cost: 2,
        lanes: 50,
        hash_length: 256,
        ..Default::default()
    };
    let pic_path = if let Some(img_name) = form.upfp_pic.file_name {
        format!(
        "/home/walker/rust/projects/Goals/goals_backend/files/{}-{}",
        GenString::new().gen_string(5, 20),
        img_name)
    } else {
        return HttpResponse::BadRequest().json(Resp::new("Sorry You have to provide the name of the image!"));
    };
    if form.upfp_pic.file.persist(&pic_path).is_err() {
        return HttpResponse::InternalServerError().json(Resp::new(
            "Sorry We're having some problem in saving you profile image!",
        ));
    }
    match hash_encoded(form.password.as_bytes(), rand_salt.as_bytes(), &arg_cfg) {
        Ok(hash) => {
            let user_info = DBUserInfo {
                username: form.username.to_string(),
                fullname: form.fullname.0,
                password: hash,
                up_posts: vec![],
                pic_path,
            };
            match db
                .create::<Option<DBUserInfo>>(("user", form.username.to_string()))
                .content(user_info.to_owned())
                .await
            {
                Ok(resl) => {
                    if resl.is_none() {
                        return HttpResponse::InternalServerError().json(Resp::new(
                            "Sorry we're having some problem when creating your account!",
                        ));
                    }
                    let exp = (Utc::now() + Duration::days(9999999)).timestamp() as usize;
                    let claims = Claims {
                        username: form.username.0,
                        password: form.password.0,
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
                Err(_) => HttpResponse::InternalServerError().json(Resp::new(
                    "Sorry we're having some problem when creating your account!"
                )),
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
