//! src/main.rs
use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // The `with` method is provided by `SubscriberExt`, an extension // trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
