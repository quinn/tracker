// use rocket::http::uri::Path;
use std::path::Path;
// use rocket::http::uri::fmt::Path;

use rocket::fs::NamedFile;
use std::path::PathBuf;

// use rocket_contrib::json::Json;
// use rocket::form::FromForm;
use rocket::{get, serde::json::Json, routes};
use schemars;
use schemars::JsonSchema;
// use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes};
use serde::{Deserialize, Serialize};

#[get("/<file..>")]
pub async fn index(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("public/").join(file);

    if !path.is_dir() && path.is_file() {
        return NamedFile::open(&path).await.ok();
    }

    let index = path.join("index.html");

    if index.is_file() {
        return NamedFile::open(index).await.ok();
    }

    let js_file = match path.to_str() {
        Some(x) => x.to_owned() + ".js",
        None => return None,
    };

    NamedFile::open(js_file).await.ok()
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
fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

#[rocket::main]
async fn main() {
    let result = rocket::build()
        .mount(
            "/",
            routes![index]
        )
        .mount(
            "/api/",
            openapi_get_routes![
                get_all_users
            ]
        ).launch().await;

    match result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    }
}
