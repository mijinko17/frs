use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};

use crate::logic::process_users_post::process_users_post;

#[derive(Deserialize, Serialize)]
pub struct UserPostBody {
    pub name: String,
    pub emotion: String,
}

pub fn users_post() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: UserPostBody| {
            process_users_post(body);
            warp::reply()
        })
}
