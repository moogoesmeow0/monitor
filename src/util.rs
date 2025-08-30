use chrono::prelude::*;
use csv::Reader;
use crate::shared::{SharedState, new_shared_state};

#[derive(Debug)]
pub enum Error {
    Generic(String),
    FileRemoved,
    ImageSizeError(String),
    StateGuardError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "Error: {}", msg),
            Error::FileRemoved => write!(f, "File was removed"),
            Error::ImageSizeError(msg) => write!(f, "Image size error, {}", msg),
            Error::StateGuardError => write!(f, "Shared state access error"),
        }
    }
}

impl std::error::Error for Error {}

pub fn read() -> Result<Vec<(f64, f64, Option<DateTime<Utc>>)>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(constants::DATA_PATH)?;
    let mut points: Vec<(f64, f64, Option<DateTime<Utc>>)> = Vec::new();

    for result in reader.records() {
        let record: csv::StringRecord = result?;
        if record.len() >= 2 {
            let x: f64 = record[0].parse()?;
            let y: f64 = record[1].parse()?;
            let time = if let Some(time_str) = record.get(2) {
                            NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S%.f")
                                .map(|ndt| Utc.from_utc_datetime(&ndt))
                                .ok()
                        } else {
                            None
                        };
            points.push((x, y, time));
        }
    }

    Ok(points)
}
pub mod constants {
    pub const DATA_PATH: &str = "data.csv";
    pub const CAM_HEIGHT: f64 = 1.0;
    pub const CAM_ANGLE: f64 = 45.0;
    pub const VIEW_WIDTH: f64 = 640.0;
    pub const VIEW_HEIGHT: f64 = 480.0;
    pub const FOV: f64 = 90.0;
}
