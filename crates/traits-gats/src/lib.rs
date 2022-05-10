// GATs are still unstable feature, but there are some crates that assume
// they'll be stabilized.
#![feature(generic_associated_types)]

/// Lifetime GATs
pub mod lt_gat;
