use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::logic::process_users_get::process_users_get;

#[derive(Deserialize, Serialize)]
pub struct UserGetResponse {
    pub name: String,
    pub emotion: String,
}

pub fn users_get() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("users")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .map(|name: String| {
            let emotion = process_users_get(&name).unwrap_or_else(|| "neutral".to_string());
            warp::reply::json(&UserGetResponse { name, emotion })
        })
}
