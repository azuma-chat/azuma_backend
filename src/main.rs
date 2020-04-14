mod api;
mod model;
mod rejection;
mod user;
mod util;

use dotenv::dotenv;
use lazy_static::lazy_static;
use log::info;
use mongodb::{Client, Database};
use std::{env, net::SocketAddr};
use tokio::{signal, sync::oneshot, task};
#[cfg(unix)]
use tokio::{
    signal::unix::{self, SignalKind},
    stream::StreamExt,
};

lazy_static! {
    static ref AZUMA_DB: Database = {
        let db_client = Client::with_uri_str(
            &env::var("AZUMA_MONGODB").expect("Environment variable AZUMA_MONGODB not found"),
        )
        .expect("Error creating MongoDB client");
        db_client.database(
            &env::var("AZUMA_DBNAME").expect("Environment variable AZUMA_DBNAME not found"),
        )
    };
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let listen_addr: SocketAddr = env::var("AZUMA_HOST")
        .expect("Environment variable AZUMA_HOST not found")
        .parse()
        .expect("Couldn't parse AZUMA_HOST");
    let (tx, rx) = oneshot::channel();
    let (addr, server) = warp::serve(api::api()).bind_with_graceful_shutdown(listen_addr, async {
        rx.await.ok();
    });
    task::spawn(server);
    info!("Listening on {}", addr);

    #[cfg(not(unix))]
    signal::ctrl_c()
        .await
        .expect("Couldn't listen to CTRL-C signal");
    #[cfg(unix)]
    {
        let sigint =
            unix::signal(SignalKind::interrupt()).expect("Couldn't listen to sigint signal");
        let sigterm =
            unix::signal(SignalKind::terminate()).expect("Couldn't listen to sigterm signal");
        let mut shutdown_signal = sigint.merge(sigterm);
        shutdown_signal.next().await;
    }
    let _ = tx.send(());
}
