use tokio::{signal, sync::oneshot};

#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;
pub mod game;

#[tokio::main]
pub async fn main() {
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        signal::ctrl_c().await
            .expect("Failed to listen for ^C");
        
        println!("Received SIGINT, shutting down...");
        
        shutdown_tx.send(())
            .expect("Failed to signal for shutdown");
    });

    loop {
        if let Ok(_) = shutdown_rx.try_recv() {
            println!("Finished queued work");
            break;
        }

        game::play("/home/zac/dev/tournament-infra/runner/hello".to_string(), "".to_string()).await;

        break;
    }
}
