use actix_web::{get, HttpResponse};

use crate::structures::{DB, DBPost, Resp};

#[get("/fetch_posts")]
pub async fn fetch_posts() -> HttpResponse {
    let db = DB.get().await;
    if db.use_ns("ns").use_db("db").await.is_err() {
       return HttpResponse::InternalServerError().json(Resp::new("Sorry We are having some problem when opening our database!"));
    }
    
    let query = "SELECT * FROM post ORDER BY RAND() LIMIT 30;";

    match db.query(query).await {
        Ok(mut resp) => {
            match resp.take::<Vec<DBPost>>(0) {
                Ok(posts) => {
                    HttpResponse::Ok().json(posts)
                },
                Err(_) => {
                    HttpResponse::InternalServerError().json(Resp::new("Something went wrong while we're getting posts!!"))
                }
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(Resp::new("Something went wrong while we're getting posts!!"))
        },
    }

}
