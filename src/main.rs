use face_reader_server::routing::routing;

#[tokio::main]
async fn main() {
    warp::serve(routing()).run(([127, 0, 0, 1], 3030)).await;
}
