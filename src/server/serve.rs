use rocket::fs::NamedFile;
use rocket::{State, get, launch, routes};
use std::path::Path;

#[get("/")]
async fn serve_image() -> Option<NamedFile> {
    NamedFile::open(Path::new("./image.jpeg")).await.ok()
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![serve_image])
}

