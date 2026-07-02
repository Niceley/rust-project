use crate::map::Position;
use crate::resource::ResourceKind;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Clone)]
pub enum Message {
    ResourceDiscovered { pos: Position, kind: ResourceKind },
    ObstacleDiscovered { pos: Position },
}

pub type MessageSender = Sender<Message>;
pub type MessageReceiver = Receiver<Message>;

#[must_use]
pub fn message_channel() -> (MessageSender, MessageReceiver) {
    channel()
}