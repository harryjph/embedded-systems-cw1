use std::convert::Infallible;
use std::sync::{Arc};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use warp::Filter;
use crate::config::Config;

mod entities;

pub fn launch(config: Config, data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> JoinHandle<()> {
    tokio::spawn(start_server(config, data_list))
}

async fn start_server(config: Config, data_list: Arc<RwLock<Vec<(f32, f32)>>>) {
    println!("Starting HTTP Server on http://localhost:{}", config.network.http_port);
    warp::serve(routes(data_list))
        .run(([0, 0, 0, 0], config.network.http_port))
        .await;
}

fn with_data_list(data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> impl Filter<Extract = (Arc<RwLock<Vec<(f32, f32)>>>,), Error = Infallible> + Clone {
    warp::any().map(move || data_list.clone())
}

fn routes(data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_data_route(data_list)
        .or(get_all_bins_route())
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

fn get_all_bins_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("bins")
        .and(warp::get())
        .and_then(get_all_bins_handler)
}

pub async fn get_all_bins_handler() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&entities::dummy_data()))
}
