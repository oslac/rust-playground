use crate::message::{InMsg, ReadEvent, WriteEvent};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub data: Option<String>,
    value: Option<String>,
}

impl From<Cli> for InMsg {
    fn from(args: Cli) -> Self {
        if args.data.is_some() & args.value.is_some() {
            let key = args.data.unwrap();
            let value = args.value.unwrap();
            InMsg::Write(WriteEvent { key, value })
        } else {
            let key = args.data.unwrap();
            InMsg::Read(ReadEvent { key })
        }
    }
}
