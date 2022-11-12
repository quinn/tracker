use rocket::fs::NamedFile;
use rocket::get;
use std::path::Path;
use std::path::PathBuf;

static BASE_PATH: &str = "../web/dist/";

#[get("/<file..>", rank = 2)]
pub async fn assets(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new(BASE_PATH).join(file);

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
