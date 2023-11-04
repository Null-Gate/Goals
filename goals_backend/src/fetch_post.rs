use actix_web::{get, HttpResponse};

use crate::structures::{DB};

#[get("/fetch_posts")]
pub async fn fetch_posts() -> HttpResponse {
    let db = DB.get().await;
    db.use_ns("ns").use_db("db").await.unwrap();
    
    let query = "SELECT * FROM post LIMIT 30;";

    let resp = db.query(query).await.unwrap();

    let query2 = "SELECT * FROM post;";

    let resp2 = db.query(query2).await.unwrap();

    println!("{resp:?}\n\n\n\n\n\n{resp2:?}");

    HttpResponse::Ok().await.unwrap()
}
