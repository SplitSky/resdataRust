use crate::model::datamodels::MyDocument;

use actix_web::{post, web, HttpResponse, Responder};
use mongodb::Client;
use serde_json::json;
#[post("/insert")]
pub async fn insert_dataset(
    client: web::Data<Client>,
    json_body: web::Json<MyDocument>,
) -> impl Responder {
    let db = client.database("Your_database_name");
    let collection = db.collection::<MyDocument>("my_collection");

    let new_doc = MyDocument {
        id: None, // TODO: implement in all of them Euan trick
        ..json_body.into_inner()
    };

    match collection.insert_one(new_doc, None).await {
        Ok(result) => HttpResponse::Ok().json(json!({"inserted_id": result.inserted_id})),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert document: {}", e))
        }
    }
}
