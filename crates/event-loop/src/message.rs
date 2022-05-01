#[derive(Debug)]
pub enum Message {
    In(InMsg),
    Out(OutMsg),
}

#[derive(Debug)]
pub struct WriteEvent {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct ReadEvent {
    pub key: String,
}

#[derive(Debug)]
pub enum InMsg {
    Read(ReadEvent),
    Write(WriteEvent),
}

#[derive(Debug)]
pub enum OutMsg {
    Read(Option<String>),
    Wrote(Option<String>),
    HttpRequest(String, Vec<u8>),
}
