use sqlx::migrate::MigrateDatabase;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DATABASE_FILENAME: &str = "pearlden.db";

#[tokio::main]
async fn main() {
    // parse command-line arguments
    let matches = clap::command!()
        .arg(
            clap::Arg::new("den")
                .required(true)
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Path to directory of den")
        )
        .arg(
            clap::Arg::new("create")
                .short('c')
                .long("create")
                .action(clap::ArgAction::SetTrue)
                .help("Create database file if absent")
        )
        .get_matches();
    let den_path = matches.get_one::<std::path::PathBuf>("den").expect("<den> should be required");
    // trace
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_new(format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME")
            )).expect("filter directives should be valid")
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // verify <den> is a directory
    if !den_path.is_dir() {
        tracing::error!("`{}` is not a directory!", den_path.display());
        std::process::exit(1);
    }
    tracing::info!("verified `{}` is a directory", den_path.display());
    // guarantee <den> contains database file
    let db_path = den_path.join(DATABASE_FILENAME);
    let db_url = format!("sqlite://{}", db_path.display());
    if !sqlx::Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        if !matches.get_flag("create") {
            tracing::error!("`{}` not found!", db_path.display());
            std::process::exit(1);
        }
        if let Err(error) = sqlx::Sqlite::create_database(&db_url).await {
            tracing::error!("could not create `{}` ({})!", db_path.display(), error);
            std::process::exit(1);
        }
        tracing::info!("created `{}`", db_path.display());
    } else {
        tracing::info!("verified `{}` exists", db_path.display());
    }
    // start server
    tracing::info!("starting server");
    let app = axum::Router::new()
        .route("/", axum::routing::get(handler))
        .layer(tower_http::trace::TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> axum::response::Html<&'static str> {
    axum::response::Html("hello world!")
}
