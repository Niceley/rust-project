use crate::map::Position;
use crate::resource::ResourceKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RobotKind {
    Scout,
    Collector,
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub id: usize,
    pub kind: RobotKind,
    pub pos: Position,
    pub carrying: Option<ResourceKind>,
}

impl Robot {
    #[must_use]
    pub fn new(id: usize, kind: RobotKind, pos: Position) -> Self {
        Self {
            id,
            kind,
            pos,
            carrying: None,
        }
    }

    #[must_use]
    pub fn symbol(&self) -> char {
        match self.kind {
            RobotKind::Scout => 'x',
            RobotKind::Collector => 'o',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_symboles_sont_corrects() {
        let scout = Robot::new(0, RobotKind::Scout, (1, 1));
        let collector = Robot::new(1, RobotKind::Collector, (2, 2));
        assert_eq!(scout.symbol(), 'x');
        assert_eq!(collector.symbol(), 'o');
    }

    #[test]
    fn un_robot_neuf_ne_porte_rien() {
        let robot = Robot::new(0, RobotKind::Collector, (0, 0));
        assert_eq!(robot.carrying, None);
    }
}
