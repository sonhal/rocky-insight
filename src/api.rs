use std::convert::Infallible;
use std::iter::Map;
use warp::{Filter, Rejection};
use warp::filters::path::Exact;
use crate::store::RockyStore;


pub fn root() -> impl Filter<Extract=impl warp::Reply, Error=Rejection> + Clone {
    warp::path::end()
        .map(|| format!("Rocky Insight"))
}

pub fn containers_status(store: RockyStore) -> impl Filter<Extract=impl warp::Reply, Error=Rejection> + Clone {
    warp::path!("containers-status")
        .map(move ||
            match store.clone().get("containers-status") {
                None => "{}".to_string(),
                Some(data) => data.to_string(),
            }
        )
}

pub fn kafka_disk_status(store: RockyStore) -> impl Filter<Extract=impl warp::Reply, Error=Rejection> + Clone {
    warp::path!("kafka-disk-status")
        .map(move ||
            match store.clone().get("kafka-disk-status") {
                None => "{}".to_string(),
                Some(data) => data.to_string(),
            }
        )
}

