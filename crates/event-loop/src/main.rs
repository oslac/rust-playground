use event_loop::instr;
use event_loop::{cli::Cli, main_loop, message::Message};

use std::io;
use std::sync::mpsc;
use std::{process::exit, thread::spawn};

use clap::StructOpt;

fn main() {
    let _ = instr::init();
    let _ = color_eyre::install();

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    spawn(move || loop {
        input(&tx2);
    });

    if let Err(e) = main_loop(tx, rx) {
        eprintln!("ERROR: while running the event loop: {}", e);
        exit(1);
    }
}

fn input(tx2: &mpsc::Sender<Message>) {
    log::debug!("Waiting for READ or WRITE message");

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("READ THE LINE");

    let args: Vec<&str> = buf.trim_end().split(' ').collect();
    let cli = Cli::parse_from(args);
    log::debug!("CLI: {:#?}", cli);

    tx2.send(Message::In(cli.into())).unwrap();
}
