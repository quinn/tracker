// use rocket::http::uri::Path;
use std::path::Path;
// use rocket::http::uri::fmt::Path;

// use rocket_contrib::json::Json;
// use rocket::form::FromForm;
use rocket::{get, routes, serde::json::Json};
use schemars;
use schemars::JsonSchema;
// use rocket_okapi::settings::UrlObject;
use rocket::response::content;
use rocket_okapi::{openapi, openapi_get_routes};
use sea_orm_rocket::Database;
use serde::{Deserialize, Serialize};
use yew::ServerRenderer;
//use rocket_cors::{AllowedHeaders, AllowedOrigins};

use sea_orm_rocket::Connection;

mod routes;
use routes::*;

mod pool;
use pool::Db;

mod models;

static BASE_PATH: &str = "../web/dist/";

#[get("/", rank = 1)]
pub async fn index() -> content::RawHtml<String> {
    let renderer = ServerRenderer::<tracker_web::components::App>::new();
    let rendered = renderer.render().await;

    let path = Path::new(BASE_PATH).join("index.html");

    let index_html = tokio::fs::read_to_string(path)
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) =
        index_html.split_once("<!-- SPLIT_HERE -->").unwrap();
    let index_html_before = index_html_before.to_owned();
    let index_html_after = index_html_after.to_owned();

    content::RawHtml(index_html_before + &rendered + &index_html_after)
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    username: String,
    #[schemars(example = "example_email")]
    email: Option<String>,
}

fn example_email() -> &'static str {
    "test@example.com"
}

/// # Get all users
///
/// Returns all users in the system.
#[openapi]
#[get("/user")]
fn get_all_users(conn: Connection<'_, Db>) -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

#[rocket::main]
async fn main() {
    let result = rocket::build()
        .attach(Db::init())
        .mount("/", routes![assets, index])
        .mount(
            "/api/",
            openapi_get_routes![get_all_users, tasks, create_task],
        )
        .launch()
        .await;

    match result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    }
}
