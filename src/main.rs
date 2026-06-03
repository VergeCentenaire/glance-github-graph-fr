use glance_github_graph_fr::api::run_api_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    run_api_server().await
} 
