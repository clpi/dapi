use serde::{de::DeserializeOwned, Serialize  };
use async_std::{io, channel::{Sender, Receiver}};

pub enum Msg {
    Text(String),
    Binary(Vec<u8>),
    Ping,
}

impl From<String> for Msg {
    fn from(inp: String) -> Self {
        Msg::Text(inp)
    }
}

impl From<Vec<u8>> for Msg {
    fn from(bytes: Vec<u8>) -> Self {
        Msg::Binary(bytes)
    }
}

pub struct Websocket((Sender<Msg>, Receiver<Msg>));

impl Websocket {
    pub fn is_open(&self) -> bool { false }

    async fn send(&self, msg: impl Into<Msg>) -> io::Result<Msg> {
        Ok(msg.into())
    }

    async fn send_json(&self, msg: &impl Serialize) -> io::Result<()> {
        Ok(())
    }


    async fn recv(&self) -> io::Result<Msg> {
        Ok(Msg::Ping)
    }

    async fn recv_json<T: DeserializeOwned>(&self, t: T) -> io::Result<()> {
        Ok(())
    }
}
