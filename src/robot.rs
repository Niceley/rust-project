use crate::map::{Map, Position};
use crate::resource::ResourceKind;
use rand::Rng;

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

    pub fn step(&mut self, map: &Map, rng: &mut impl Rng) {
        match self.kind {
            RobotKind::Scout => self.wander(map, rng),
        }
    }

    fn wander(&mut self, map: &Map, rng: &mut impl Rng) {
        const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let (dx, dy) = DIRS[rng.gen_range(0..DIRS.len())];
        let nx = i32::from(self.pos.0) + dx;
        let ny = i32::from(self.pos.1) + dy;
        if nx < 0 || ny < 0 {
            return;
        }
        let (nx, ny) = (nx as u16, ny as u16);
        if map.is_walkable(nx, ny) {
            self.pos = (nx, ny);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Tile;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

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

    #[test]
    fn un_eclaireur_n_entre_pas_dans_un_obstacle() {
        let mut map = Map::new(3, 3);
        for (x, y) in [(0,0),(1,0),(2,0),(0,1),(2,1),(0,2),(1,2),(2,2)] {
            map.set(x, y, Tile::Obstacle);
        }
        let mut rng = StdRng::seed_from_u64(0);
        let mut scout = Robot::new(0, RobotKind::Scout, (1, 1));
        for _ in 0..20 {
            scout.step(&map, &mut rng);
        }
        assert_eq!(scout.pos, (1, 1));
    }

}
