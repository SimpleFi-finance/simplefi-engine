use chrono::Utc;
use log::LevelFilter;
use fern::Dispatch;

pub fn init_logging() {
    let global_settings = simp_settings::load_settings().expect("Failed to load settings");

    let binding = global_settings.log_level.to_lowercase();
    let log_level = binding.as_str();
    let log_file = global_settings.log_file;

    let log_level_selected = match log_level {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let base_config = Dispatch::new()
        .level(log_level_selected)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                Utc::now().timestamp_millis(),
                record.target(),
                record.level(),
                message
            ))
        });

    let console_output = Dispatch::new()
        .chain(std::io::stdout());

    let file_config = | file: &str | {
        Dispatch::new()
            .chain(fern::log_file(file).expect("Failed to open log file"))
    };


    if log_file.is_empty() {
        base_config.chain(console_output).apply().expect("Failed to initialize logging");
    } else {
        base_config.chain(file_config(&log_file)).apply().expect("Failed to initialize logging");
    }
}
