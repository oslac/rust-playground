use global_state::GlobalState;
use message::Message;

use std::sync::mpsc::{Receiver, Sender};

pub mod cli;
pub mod effects;
pub mod global_state;
pub mod instr;
pub mod message;

pub type EventLoopResult = Result<(), color_eyre::Report>;

#[tracing::instrument(skip(sender, receiver), name = "Main Loop")]
pub fn main_loop(sender: Sender<Message>, receiver: Receiver<Message>) -> EventLoopResult {
    log::debug!("Starting a new event loop");
    GlobalState::new(sender).run(receiver)
}
