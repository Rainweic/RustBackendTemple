use clap::Parser;
use std::process::exit;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ApiTemple::app_state::AppState;
use ApiTemple::config::get_config;
use ApiTemple::db::{init_db_pool, ping_db};
use ApiTemple::routes::routes;
use tracing_appender::rolling::{daily, RollingFileAppender};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() {

    let file_appender = daily("./logs", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer()
            .with_writer(non_blocking.and(std::io::stdout)))
        .init();

    log::info!(
        "The server is starting up. Its process id is {}.",
        std::process::id()
    );

    let app_cfg = get_config().expect("Failed to load the app config.");

    let db_conn_pool = init_db_pool(&app_cfg)
        .await
        .expect("Failed to connect to database.");
    match ping_db(&db_conn_pool).await {
        true => log::info!(
            "Connected to the database (with {} conns).",
            db_conn_pool.size()
        ),
        false => {
            log::error!("Failed to ping the database. Exiting now.");
            exit(1);
        }
    }

    let state = AppState::new(db_conn_pool);

    let routes = routes(state);

    let addr = "0.0.0.0:8080";
    log::info!("Listening for requests on http://{} ...", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Unable to start server");
}

async fn shutdown_signal() {
    //
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to init Ctrl+C handler")
    };

    #[cfg(unix)]
    use tokio::signal::unix;
    let terminate = async {
        unix::signal(unix::SignalKind::terminate())
            .expect("Failed to init signal handler")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    log::info!("Shutting down gracefully ...")
}

#[derive(Parser, Debug)]
#[clap(
    name = "server",
    about = "The server side of Fullstack Rust RealWorld App solution."
)]
struct Opt {
    /// The HTTP listening address.
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// The HTTP listening port.
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// The logging level.
    #[clap(short = 'l', long = "log", default_value = "info")]
    log_level: String,

    /// The directory where assets (static) files are served from. <br/>
    /// These assets are fetched by requests using `/assets/*` path.
    #[clap(short = 's', long = "assets-dir", default_value = "../dist")]
    assets_dir: String,
}
