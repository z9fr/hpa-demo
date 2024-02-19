use axum::Json;
use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateJobRequest {
    pub name: String,
}

#[instrument()]
pub async fn index(Json(payload): Json<CreateJobRequest>) -> Json<JobAddedResult> {
    HIT_COUNTER.inc();

    let result = JobAddedResult {
        status: String::from("success"),
        name: payload.name,
    };

    Json(result)
}
