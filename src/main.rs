use std::path::PathBuf;

const DATABASE_FILENAME: &str = "pearlden.db";

fn main() {
    // parse command-line arguments
    let matches = clap::command!()
        .arg(
            clap::Arg::new("den")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
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
    println!("opening \"{}\"...", db_path.display());
    // placeholder for opening database file
    if !db_path.is_file() {
        eprintln!("error: \"{}\" is not a file!", db_path.display());
        std::process::exit(1);
    }
}
