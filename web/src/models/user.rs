/*
 * tracker_server
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */


use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl User {
    pub fn new(user_id: i32, username: String) -> User {
        User {
            user_id,
            username,
            email: None,
        }
    }
}


