use csv::Reader;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    FileRemoved,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(msg) => write!(f, "Error: {}", msg),
            Error::FileRemoved => write!(f, "File was removed"),
        }
    }
}

impl std::error::Error for Error {}


pub fn read() -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(DATA_PATH)?;
    let mut points: Vec<(i32, i32)> = Vec::new();

    for result in reader.records() {
        let record: csv::StringRecord = result?;
        if record.len() >= 2 {
            let x: i32 = record[0].parse()?;
            let y: i32 = record[1].parse()?;
            points.push((x, y));
        }
    }

    Ok(points)
}

pub const DATA_PATH: &str = "data.csv";
pub const CAM_HEIGHT: f64 = 1.0;
pub const CAM_ANGLE: f64 = 45.0;
pub const VIEW_WIDTH: f64 = 640.0;
pub const VIEW_HEIGHT: f64 = 480.0;
pub const FOV: f64 = 90.0;