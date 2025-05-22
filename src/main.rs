use sqlx::migrate::MigrateDatabase;
use std::path::PathBuf;

const DATABASE_FILENAME: &str = "pearlden.db";

#[tokio::main]
async fn main() {
    // parse command-line arguments
    let matches = clap::command!()
        .arg(
            clap::Arg::new("den")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
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
    let den_path = matches.get_one::<PathBuf>("den").expect("<den> should be required");
    // attach to den
    println!("verifying \"{}\" is a directory...", den_path.display());
    if !den_path.is_dir() {
        eprintln!("error: \"{}\" is not a directory!", den_path.display());
        std::process::exit(1);
    }
    let db_path = den_path.join(DATABASE_FILENAME);
    let db_url = format!("sqlite://{}", db_path.display());
    println!("verifying \"{}\" exists...", db_path.display());
    if !sqlx::Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        if !matches.get_flag("create") {
            eprintln!("error: \"{}\" does not exist!", db_path.display());
            std::process::exit(1);
        }
        println!("creating \"{}\"...", db_path.display());
        if let Err(error) = sqlx::Sqlite::create_database(&db_url).await {
            eprintln!("error: could not create \"{}\" ({})!", db_path.display(), error);
            std::process::exit(1);
        }
    }
    // start server
    println!("starting server...");
    let app = axum::Router::new().route("/", axum::routing::get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> axum::response::Html<&'static str> {
    axum::response::Html("hello world!")
}
