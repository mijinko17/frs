use face_reader_server::routing::routing;

#[tokio::main]
async fn main() {
    warp::serve(routing()).run(([0, 0, 0, 0], 3030)).await;
}
