use std::convert::Infallible;
use std::sync::{Arc};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use warp::Filter;

pub fn launch(data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> JoinHandle<()> {
    tokio::spawn(start_server(data_list))
}

async fn start_server(data_list: Arc<RwLock<Vec<(f32, f32)>>>) {
    println!("Starting HTTP Server on http://localhost");
    warp::serve(routes(data_list))
        .run(([0, 0, 0, 0], 80))
        .await;
}

fn with_data_list(data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> impl Filter<Extract = (Arc<RwLock<Vec<(f32, f32)>>>,), Error = Infallible> + Clone {
    warp::any().map(move || data_list.clone())
}

fn routes(data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_data_route(data_list)
}

fn get_data_route(data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("data")
        .and(warp::get())
        .and(with_data_list(data_list))
        .and_then(get_data_handler)
}

pub async fn get_data_handler(data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> Result<impl warp::Reply, Infallible> {
    let customers = data_list.read().await;
    Ok(warp::reply::json(&*customers))
}
