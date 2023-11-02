use tracing::info;

fn main() {
    info!("Starting SIMP CLI process");

    if let Err(err) = simp::cli::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
