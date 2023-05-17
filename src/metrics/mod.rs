use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Orbit, Request, Rocket};
use serde::Serialize;
use serde::Deserialize;
use crate::DATABASE_NAME;

#[derive(Debug, Clone)]
pub struct Metrics{
    pub request: Arc<RwLock<HashMap<String, Visitor>>>
}

#[derive(Serialize, Deserialize)]
pub struct SavableMetrics{
    pub visitors: HashMap<String, Visitor>
}

impl Metrics{
    pub fn serialize_metrics(&self) -> SavableMetrics{
        let lock = self.request.read().unwrap();
        SavableMetrics{ visitors: lock.clone() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visitor{
    pub request_count: u32
}

#[rocket::async_trait]
impl Fairing for Metrics {
    fn info(&self) -> Info {
        Info{
            name: "Metrics",
            kind: Kind::Request | Kind::Shutdown,
        }
    }

    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {
        let state = _req.rocket().state::<Metrics>();
        match state {
            None => {}
            Some(s) => {
                let ip_address = _req.remote();
                match ip_address {
                    None => {}
                    Some(ip) => {
                        let mut lock = s.request.write().unwrap();
                        match lock.get_mut(&ip.ip().to_string()) {
                            None => {
                                // New visitor
                                lock.insert(ip.ip().to_string(), Visitor{ request_count: 1 });
                            }
                            Some(user) => {
                                // Visitor seen before
                                user.request_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    async fn on_shutdown(&self, _rocket: &Rocket<Orbit>) {
        // let file = match File::open(file_name){
        //     Ok(f) => {f}
        //     Err(_) => {
        //         File::create(file_name).unwrap();
        //     }
        // };

        let ser = serde_json::to_string(&self.serialize_metrics()).unwrap();
        let mut database_file = File::create(DATABASE_NAME).unwrap();
        database_file.write_all(ser.as_ref()).unwrap();
    }
}