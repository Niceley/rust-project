use crate::comms::Message;
use crate::map::Position;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Base {
    pub pos: Position,
    pub known_resources: HashSet<Position>,
    pub known_obstacles: HashSet<Position>,
}

impl Base {
    #[must_use]
    pub fn new(pos: Position) -> Self {
        Self { pos, ..Self::default() }
    }

    pub fn handle_message(&mut self, msg: &Message) {
        match msg {
            Message::ResourceDiscovered { pos, .. } => { self.known_resources.insert(*pos); }
            Message::ObstacleDiscovered { pos } => { self.known_obstacles.insert(*pos); }
        }
    }
}
