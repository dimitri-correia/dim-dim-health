#[tokio::main]
async fn main() {
    dimdim_health_worker::worker_main::worker::worker_main().await;
}
