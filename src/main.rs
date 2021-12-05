use std::borrow::BorrowMut;
use std::io::Read;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use log::{info, LevelFilter, trace, warn};
use tokio::try_join;
use warp::Filter;

use rapids_and_rivers::config::Config;

use crate::api::{containers_status, kafka_disk_status, root};
use crate::containers::container_listener::container_river;
use crate::kafka_disk_use::kafka_disk_listener::kafka_disk_use_river;
use crate::rapids_and_rivers::kafka;
use crate::rapids_and_rivers::rapid::Rapid;
use crate::store::RockyStore;

#[macro_use]
extern crate log;

mod api;
mod containers;
mod kafka_disk_use;
mod rapids_and_rivers;
mod store;

const BOOTSTRAP_SERVERS: &str = "localhost:29092";
const TOPIC: &str = "osquery_topic";

#[tokio::main]
async fn main() {
    env_logger::try_init();

    let config = Config::from_env();
    info!("Rocky Insight starting with config = {:#?}", config);

    let mut store = RockyStore::new();

    let mut rapid = Rapid::new(&config);
    rapid.register(container_river(&store));
    rapid.register(kafka_disk_use_river(&store));
    trace!("Rapid created");

    let routes = root()
        .or(containers_status(store.clone()))
        .or(kafka_disk_status(store.clone()));

    let web_handle =
        tokio::spawn(async move { warp::serve(routes).run(([0, 0, 0, 0], 3030)).await });

    let rapid_handle = tokio::spawn(async move { rapid.start().await });

    try_join!(web_handle, rapid_handle);
    // result.map_err( |err| error!(err));
    info!("Stopping...")
}