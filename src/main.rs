mod api;
mod db;
mod model;
mod rejection;
mod user;

use crate::db::create_pool;
use dotenv::dotenv;
use log::info;
use std::{env, net::SocketAddr};
#[cfg(not(unix))]
use tokio::signal;
#[cfg(unix)]
use tokio::{
    signal::unix::{self, SignalKind},
    stream::StreamExt,
};
use tokio::{sync::oneshot, task};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    let pool = create_pool().await;

    let listen_addr: SocketAddr = env::var("AZUMA_HOST")
        .expect("Environment variable AZUMA_HOST not found")
        .parse()
        .expect("Couldn't parse AZUMA_HOST");
    let (tx, rx) = oneshot::channel();
    let (addr, server) =
        warp::serve(api::api(pool).await).bind_with_graceful_shutdown(listen_addr, async {
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
