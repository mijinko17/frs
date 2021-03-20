use warp::{Filter, Reply};

use self::{users_get::users_get, users_post::users_post};

pub fn routing() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(|| "Root of face-reader-server")
        .or(users_post())
        .or(users_get())
}

pub mod users_get;
pub mod users_post;
