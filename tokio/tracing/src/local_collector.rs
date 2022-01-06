use tracing::{info, Level};

fn main() {
    let collector = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .finish();

    tracing::collect::with_default(collector, || {
        info!("This will be logged to stdout");
    });
    info!("This will _not_ be logged to stdout");
}