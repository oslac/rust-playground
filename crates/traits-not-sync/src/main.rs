use std::cell::Cell;
use std::marker::PhantomData;

fn main() {
    // http://cliffle.com/blog/not-thread-safe/#the-engineering-marvel-of-sync-and-send
    println!("Until negative impls are stable, use this to explicitly opt out of Sync");
}

/// # NotSyncMarker
///
/// This is a type that can be used to opt types out of automatic ``Sync``.
/// The struct acts like it contains a ``Cell``, which is **not** ``Sync``,
/// but *does not spend any memory to actually store a ``Cell``* because it is a phantom.
///
/// ## Example
///
/// To use it for some structure to make it ``!Sync``:
/// ```rust
/// struct SomeTypeThatShouldNotBeAutomaticSync {
///    first_data: AtomicU8,
///    second_data: AtomicU8,
///    _marker: NotSyncMarker,
/// }
/// ```
#[derive(Copy, Clone, Debug, Default)]
struct NotSyncMarker(PhantomData<Cell<u8>>);
