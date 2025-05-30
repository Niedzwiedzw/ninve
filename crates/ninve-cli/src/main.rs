use {
    anyhow::{Context, Result},
    futures::FutureExt,
    tokio::task::LocalSet,
};

pub struct Ninve {}

#[allow(dead_code)]
fn in_runtime<T, F>(in_runtime: F) -> Result<T>
where
    T: 'static,
    F: AsyncFnOnce() -> Result<T> + 'static,
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building runtime")?;

    runtime.block_on(async {
        let set = LocalSet::new();
        let output = set.spawn_local(in_runtime());
        set.await;
        output
            .map(|r| {
                r.context("task crashed")
                    .and_then(|r| r.context("runtime crashed"))
            })
            .await
    })
}

mod logging {
    use {
        std::str::FromStr,
        tracing::Level,
        tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt},
    };

    /// Sets up tracing with a level from the RUST_LOG env var and output to stderr
    ///
    /// Falls back to INFO level if RUST_LOG is not set or invalid
    pub fn setup_tracing() {
        // Get the log level from RUST_LOG env var, default to INFO
        let log_level = std::env::var("RUST_LOG")
            .ok()
            .and_then(|level| Level::from_str(&level).ok())
            .unwrap_or(Level::INFO);

        // Create a filter from the log level
        let filter = EnvFilter::builder()
            .with_default_directive(log_level.into())
            .from_env_lossy();

        // Set up the formatting layer
        let fmt_layer = fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(true) // Enable colors in terminal
            .with_target(true) // Include event targets
            .with_thread_ids(false) // Optional: disable thread IDs
            .with_thread_names(false); // Optional: disable thread names

        // Install the subscriber
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
}

fn main() -> Result<()> {
    logging::setup_tracing();
    Ok(())
}
