use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use above_api_rs::create_app;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

struct ServerConfig {
    host: IpAddr,
    port: u16,
}

mod consts;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .pretty()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let server_config = load_configs();

    let app = create_app();

    let addr = SocketAddr::from((server_config.host, server_config.port));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn load_configs() -> ServerConfig {
    dotenv::dotenv().ok();

    return ServerConfig {
        host: match env::var("HOST") {
            Ok(value) => value
                .parse::<IpAddr>()
                .unwrap_or_else(|_| consts::DEFAULT_HOST),
            Err(_) => consts::DEFAULT_HOST,
        },

        port: match env::var("PORT") {
            Ok(value) => value
                .parse::<u16>()
                .unwrap_or_else(|_| consts::DEFAULT_PORT),
            Err(_) => consts::DEFAULT_PORT,
        },
    };
}
