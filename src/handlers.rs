use std::thread::current;

use axum::Json;
use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter};
use serde::{Deserialize, Serialize};
use tokio::task;
use tracing::instrument;

lazy_static! {
    static ref HIT_COUNTER: IntCounter =
        register_int_counter!("jobs", "number of jobs running").unwrap();
}

#[derive(Serialize)]
pub struct JobAddedResult {
    pub status: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateJobRequest {
    pub name: String,
}

#[instrument()]
pub async fn index(Json(payload): Json<CreateJobRequest>) -> Json<JobAddedResult> {
    HIT_COUNTER.inc();

    task::spawn(async move {
        log::info!("running background job");
        loop {
            // Perform some CPU-intensive operation
            let mut result: u64 = 0;
            for i in 0..1_000_000 {
                result += i;
            }

            // Introduce some delay to reduce the CPU usage
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    });

    let result = JobAddedResult {
        status: String::from("success"),
        name: payload.name,
    };

    Json(result)
}
