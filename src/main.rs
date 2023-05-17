#[macro_use] extern crate rocket;

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use rocket::{Build, Rocket};
use crate::metrics::{Metrics, SavableMetrics};

mod metrics;
pub static DATABASE_NAME: &str = "costi-online-professional.ser";

#[launch]
fn rocket() -> Rocket<Build> {
    let mut database_file = match File::open(DATABASE_NAME) {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s);
            match serde_json::from_str::<SavableMetrics>(&s) {
                Ok(sm) => {sm}
                Err(_) => {SavableMetrics{ visitors: Default::default() }}
            }
        }
        Err(_) => {SavableMetrics{ visitors: Default::default()}}
    };
    let metrics = Metrics{ request: Arc::new(RwLock::new(database_file.visitors)) };
    rocket::build()
        .manage(metrics.clone())
        .attach(metrics)
        .mount("/", routes![index, other])
}

#[get("/<costi>")]
fn index(costi: u8) -> String {
    format!("{costi}")
}

#[get("/Axcel/<name>")]
fn other(name: String) -> String {
    format!("Axcel says hi to {name}")
}