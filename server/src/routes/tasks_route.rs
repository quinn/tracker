use crate::models;
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
) -> Result<Json<Vec<models::Task>>, (Status, Json<ErrorMessage>)> {
    let db = conn.into_inner();

    let result = task::Entity::find().all(db).await;

    match result {
        Ok(tasks) => return Ok(Json(tasks.into_iter().map(|t| t.into()).collect())),
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(ErrorMessage {
                    message: "could not query database".into(),
                }),
            ))
        }
    }
}
