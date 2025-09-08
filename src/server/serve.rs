use crate::shared::{SharedData, SharedState};
use rocket::fs::NamedFile;
use rocket::{
    State, get, launch, routes,
    serde::json::{Json, serde_json},
};
use std::path::Path;

#[get("/")]
async fn serve_image() -> Option<NamedFile> {
    NamedFile::open(Path::new("./output.png")).await.ok()
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
                "point_count": data.points.len() + 1,
                "last_updated": data.last_updated
            })));
        }
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[get("/data/<index>")]
fn get_item(index: usize, shared_state: &State<SharedState>) -> Option<Json<(f64, f64, String)>> {
    let data = shared_state.read().ok()?;
    data.points
        .get(index)
        .cloned()
        .and_then(|f| f.2.map(|s| (f.0, f.1, s.to_string())))
        .map(Json)
}

#[get("/data/<begin>/<end>")]
fn get_range(
    begin: usize,
    end: usize,
    shared_state: &State<SharedState>,
) -> Option<Json<Vec<(f64, f64, String)>>> {
    let data = shared_state.read().ok()?;

    if begin >= end {
        return None;
    };

    let points = data.points.get(begin..end)?;
    let result: Vec<(f64, f64, String)> = points
        .iter()
        .filter_map(|p| p.2.as_ref().map(|s| (p.0, p.1, s.clone().to_string())))
        .collect();
    
    Some(Json(result))
}

pub fn rocket(shared_state: SharedState) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(shared_state)
        .mount("/", routes![serve_image, get_data, get_stats, get_range, get_item])
}
