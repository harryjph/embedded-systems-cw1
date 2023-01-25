use tokio::task::JoinHandle;
use warp::Filter;

pub fn launch() -> JoinHandle<()> {
    tokio::spawn(start_server())
}

async fn start_server() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    println!("Starting HTTP Server on http://localhost");
    warp::serve(hello)
        .run(([0, 0, 0, 0], 80))
        .await;
}
