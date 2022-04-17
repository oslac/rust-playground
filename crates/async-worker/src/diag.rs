use std::error::Error;

use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;
use tracing_tree::HierarchicalLayer;

pub fn setup() -> Result<(), Box<dyn Error>> {
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(HierarchicalLayer::new(2).with_targets(true).with_bracketed_fields(true))
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
