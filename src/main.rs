mod api;

use log::info;
use tokio::{signal, sync::oneshot};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let (tx, rx) = oneshot::channel();
    let (addr, server) =
        warp::serve(api::api()).bind_with_graceful_shutdown(([127, 0, 0, 1], 7373), async {
            rx.await.ok();
        });
    tokio::task::spawn(server);
    info!("Listening on {}", addr);
    signal::ctrl_c()
        .await
        .expect("Couldn't listen to CTRL-C signal");
    let _ = tx.send(());
}
