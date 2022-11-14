use rocket::{post, serde::json::Json};
use rocket_okapi::openapi;
use sea_orm_rocket::Connection;

use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::pool::Db;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct CreateTaskInput {
    pub id: Option<String>,
    pub summary: String,
}

#[openapi]
#[post("/create_task", format = "json", data = "<create_task_input>")]
pub fn create_task(
    conn: Connection<'_, Db>,
    create_task_input: Json<CreateTaskInput>,
) -> Json<String> {
    Json("".to_string())
}
