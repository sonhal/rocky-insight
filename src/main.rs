use std::borrow::BorrowMut;
use std::io::Read;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::try_join;

#[macro_use]
extern crate log;

mod containers;
mod rapids_and_rivers;
mod store;

use log::{trace, info, warn, LevelFilter};
use warp::Filter;
use crate::rapids_and_rivers::kafka;
use crate::rapids_and_rivers::rapid::Rapid;
use crate::store::RockyStore;

const BOOTSTRAP_SERVERS: &str = "localhost:29092";
const TOPIC: &str = "osquery-topic";

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).try_init();
    info!("Rocky Insight starting");

    let mut store = RockyStore::new();

    let mut rapid = Rapid::new(BOOTSTRAP_SERVERS, TOPIC);
    trace!("Consumer created");

    let mut container_store = store.clone();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let root = warp::path::end()
        .map(|| format!("Rocky Insight"));

    let containers = warp::path!("containers")
        .map(move ||
            match container_store.clone().get("containers") {
                None => "{}".to_string(),
                Some(data) => data.to_string(),
            }
        );

    let routes = root.or(containers);

    let web_handle = tokio::spawn(async move {
        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await
    });

    let rapid_handle = tokio::spawn(async move {
        rapid.start().await
    });

    try_join!(web_handle, rapid_handle);
    // result.map_err( |err| error!(err));
    info!("Stopping...")
}
