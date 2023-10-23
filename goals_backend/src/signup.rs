use actix_web::{post, web::Json, HttpResponse};
use argon2::{hash_encoded, Config, Variant, Version};
use rand::{distributions::DistString, Rng};

use crate::{structures::{SignUpInfo, DB}, gen_salt::GenString};

#[post("/sign_up")]
pub async fn sign_up(info: Json<SignUpInfo>) -> HttpResponse {
    let db = DB.get().await;
    let mut rngs = rand::thread_rng();
    let rand_salt = GenString.sample_string(&mut rngs.clone(), rngs.gen_range(20..100));
    let arg_cfg = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 655360,
        time_cost: 2,
        lanes: 20,
        hash_length: 50,
        ..Default::default()
    };
    let hash = hash_encoded(info.password.as_bytes(), rand_salt.as_bytes(), &arg_cfg);
    todo!()
}
