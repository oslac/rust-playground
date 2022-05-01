use tracing_error::ErrorLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};
use tracing_tree::HierarchicalLayer;

pub fn init() {
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(HierarchicalLayer::new(2).with_targets(true).with_bracketed_fields(true))
        .with(ErrorLayer::default())
        .init();
}
