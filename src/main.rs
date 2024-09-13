use std::process::exit;
use tracing_appender::rolling::daily;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ApiTemple::app_state::AppState;
use ApiTemple::config::get_config;
use ApiTemple::db::{init_db_pool, ping_db};
use ApiTemple::routes::routes;

#[tokio::main]
async fn main() {
    let file_appender = daily("./logs", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking.and(std::io::stdout))
                // .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
                .with_ansi(false)
                .with_target(true)
                .with_level(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true)
        )
        .init();

    tracing::info!(
        "The server is starting up. Its process id is {}.",
        std::process::id()
    );

    let app_cfg = get_config().expect("Failed to load the app config.");

    let db_conn_pool = init_db_pool(&app_cfg)
        .await
        .expect("Failed to connect to database.");
    match ping_db(&db_conn_pool).await {
        true => tracing::info!(
            "Connected to the database (with {} conns).",
            db_conn_pool.size()
        ),
        false => {
            tracing::error!("Failed to ping the database. Exiting now.");
            exit(1);
        }
    }

    let state = AppState::new(db_conn_pool);

    let routes = routes(state);

    let addr = "0.0.0.0:8080";
    tracing::info!("Listening for requests on http://{} ...", addr);

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
    tracing::info!("Shutting down gracefully ...")
}
