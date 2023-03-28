#[cfg(feature = "with-tracing")]
pub fn setup_tracing() {
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting tracing subscriber failed");
}

/* Workaround for the following feature:

#[cfg(not(feature = "with-tracing"))]
pub fn setup_tracing() {
    // Do nothing if the feature is not enabled
}
*/
