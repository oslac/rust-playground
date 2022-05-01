use crate::effects::SideEffects;
use crate::message::{InMsg, Message, OutMsg, ReadEvent, WriteEvent};
use crate::EventLoopResult;

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

type Db = HashMap<String, String>;

pub struct GlobalState {
    sender: Sender<Message>,
    db: Db,
    // ... more items as required ..
}

impl GlobalState {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { sender, db: HashMap::default() }
    }

    #[tracing::instrument(skip(self, inbox))]
    pub fn run(&mut self, inbox: Receiver<Message>) -> EventLoopResult {
        while let Some(msg) = self.next_msg(&inbox) {
            log::debug!("New message received");
            self.handle(msg)?
        }

        Ok(())
    }

    #[tracing::instrument(skip(self, mail))]
    fn handle(&mut self, mail: Message) -> EventLoopResult {
        // The handle fn. is going to explode in size out of necessity
        match mail {
            Message::In(inmsg) => match inmsg {
                InMsg::Read(event) => {
                    tracing::debug!("New Read: {:?}", event);
                    let res = self.on_read(event);
                    for outmsg in res.messages_to_send {
                        self.sender.send(Message::Out(outmsg))?
                    }
                }
                InMsg::Write(event) => {
                    tracing::debug!("New Write: {:?}", event);
                    let res = self.on_write(event);
                    for outmsg in res.messages_to_send {
                        self.sender.send(Message::Out(outmsg))?
                    }
                }
            },

            Message::Out(outmsg) => match outmsg {
                OutMsg::Read(event) => tracing::debug!("Read Result: {:?}", event),
                OutMsg::Wrote(event) => tracing::debug!("Write Result: {:?}", event),
                _ => unimplemented!(),
            },
        }

        Ok(())
    }

    /// Asks the receiver for the next message.
    #[tracing::instrument(skip(self, inbox))]
    fn next_msg(&self, inbox: &Receiver<Message>) -> Option<Message> {
        inbox.recv().ok()
    }

    #[tracing::instrument(skip(self, event))]
    fn on_write(&mut self, event: WriteEvent) -> SideEffects {
        let res = self.db.insert(event.key, event.value);
        SideEffects { messages_to_send: vec![OutMsg::Wrote(res)] }
    }

    #[tracing::instrument(skip(self, event))]
    fn on_read(&self, event: ReadEvent) -> SideEffects {
        let res = self.db.get(&event.key).cloned();
        SideEffects { messages_to_send: vec![OutMsg::Read(res)] }
    }
}
