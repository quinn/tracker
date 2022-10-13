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
use yew::ServerRenderer;
use rocket::response::{content};
//use rocket_cors::{AllowedHeaders, AllowedOrigins};


#[get("/<file..>", rank = 2)]
pub async fn assets(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("../dist/").join(file);

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


#[get("/", rank = 1)]
pub async fn index() -> content::RawHtml<String> {
    let renderer = ServerRenderer::<tracker_web::components::App>::new();
    let rendered = renderer.render().await;

    let path = Path::new("../dist/index.html");

    let index_html = tokio::fs::read_to_string(path)
    .await
    .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html
        .split_once("<!-- SPLIT_HERE -->")
        .unwrap();
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
            routes![
                assets,
                index
            ]
        )
        .mount(
            "/api/",
            openapi_get_routes![
                get_all_users
            ]
        )
        .launch()
        .await;

    match result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    }
}
