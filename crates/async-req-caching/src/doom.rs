use std::sync::atomic::AtomicU64;

/// This counter acts as a trigger to display the request coalescing
/// functionality in this example program.
pub static COUNTER: AtomicU64 = AtomicU64::new(0);
