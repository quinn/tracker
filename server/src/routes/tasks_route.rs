use crate::pool::Db;
use entity::task;
use rocket::http::Status;
use rocket::{get, serde::json::Json};
use rocket_okapi::{openapi, JsonSchema, OpenApiError};
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ErrorMessage {
    pub message: String,
}

#[openapi]
#[get("/tasks")]
pub async fn tasks(
    conn: Connection<'_, Db>,
) -> Result<Json<Vec<task::Model>>, (Status, Json<ErrorMessage>)> {
    let db = conn.into_inner();

    let result = task::Entity::find().all(db).await;

    match result {
        Ok(l) => return Ok(Json(l)),
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(ErrorMessage {
                    message: String::from("could not query database"),
                }),
            ))
        }
    }
}
