use actix_web::{get, HttpResponse};

use crate::structures::{DB, Post};

#[get("/fetch_posts")]
pub async fn fetch_posts() -> HttpResponse {
    let db = DB.get().await;
    db.use_ns("ns").use_db("db").await.unwrap();
    
    let query = "SELECT * FROM post ORDER BY RAND() LIMIT 30;";

    let resp: Vec<Post> = db.query(query).await.unwrap().take(0).unwrap();

    println!("{resp:?}");

    HttpResponse::Ok().await.unwrap()
}
