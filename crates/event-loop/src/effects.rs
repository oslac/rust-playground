use crate::message::OutMsg;

/// Any side-effect the operations may cause are returned as messages (as
/// opposed to just running them as a side-channel in the function
/// itself).
///
/// This can be easily extended by extending what `OutMsg` can return.
pub struct SideEffects {
    pub messages_to_send: Vec<OutMsg>,
}
