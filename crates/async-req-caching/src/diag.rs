use std::error::Error;

use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

pub fn setup() -> Result<(), Box<dyn Error>> {
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(HierarchicalLayer::new(2).with_targets(true).with_bracketed_fields(true))
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
