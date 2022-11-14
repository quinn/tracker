use entity::task::Model as TaskEntity;

use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Task {
    pub id: String,
    pub summary: String,
}

impl From<TaskEntity> for Task {
    fn from(entity: TaskEntity) -> Self {
        Task {
            id: entity.id.to_string(),
            summary: entity.summary,
        }
    }
}
