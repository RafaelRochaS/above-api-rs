use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use above_api_rs::create_app;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

struct ServerConfig {
    host: IpAddr,
    port: u16,
}

mod consts;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

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
