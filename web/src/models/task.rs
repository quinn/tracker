/*
 * tracker_server
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "summary")]
    pub summary: String,
}

impl Task {
    pub fn new(id: String, summary: String) -> Task {
        Task { id, summary }
    }
}
