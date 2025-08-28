use std::sync::{Arc, RwLock};
use rocket::time::{self, macros::datetime, Date, Time};
use serde::Serialize;
use chrono::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct SharedData {
    pub points: Vec<(f64, f64, Option<DateTime<Utc>>,)>,
    pub last_updated: std::time::SystemTime,
}

impl SharedData {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            last_updated: std::time::SystemTime::now(),
        }
    }

    pub fn update_points(&mut self, new_points: Vec<(f64, f64, Option<DateTime<Utc>>)>) {
        self.points = new_points;
        self.last_updated = std::time::SystemTime::now();
    }
}

pub type SharedState = Arc<RwLock<SharedData>>;

pub fn new_shared_state() -> SharedState {
    Arc::new(RwLock::new(SharedData::new()))
}