use tracing::Level;

mod yak_shave;

fn main() {
    assert!(Level::TRACE > Level::DEBUG);
    assert!(Level::ERROR < Level::WARN);
    assert!(Level::INFO <= Level::DEBUG);

    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(Level::TRACE)
        // sets this to be the default, global collector for this application.
        .init();

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    tracing::info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = yak_shave::shave_all(number_of_yaks);
    tracing::info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
}