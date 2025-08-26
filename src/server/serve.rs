use crate::shared::{SharedData, SharedState};
use rocket::fs::NamedFile;
use rocket::{
    State, get, launch, routes,
    serde::json::{Json, serde_json},
};
use std::path::Path;

#[get("/")]
async fn serve_image() -> Option<NamedFile> {
    NamedFile::open(Path::new("./output.jpeg")).await.ok()
}

#[get("/data")]
fn get_data(shared_state: &State<SharedState>) -> Result<Json<SharedData>, rocket::http::Status> {
    match shared_state.read() {
        Ok(data) => return Ok(Json(data.clone())),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[get("/stats")]
fn get_stats(
    shared_state: &State<SharedState>,
) -> Result<Json<serde_json::Value>, rocket::http::Status> {
    match shared_state.read() {
        Ok(data) => {
            return Ok(Json(serde_json::json!({
                "point_count": data.points.len(),
                "last_updated": data.last_updated
            })));
        }
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

pub fn rocket(shared_state: SharedState) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(shared_state)
        .mount("/", routes![serve_image, get_data, get_stats])
}
