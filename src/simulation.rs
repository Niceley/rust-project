use crate::config::SimConfig;
use crate::map::Map;
use crate::robot::{Robot, RobotKind};

pub struct Simulation {
    pub config: SimConfig,
    pub map: Map,
    pub robots: Vec<Robot>,
}

impl Simulation {
    #[must_use]
    pub fn new(config: SimConfig) -> Self {
        let map = Map::generate(&config);

        let scouts = (0..config.num_scouts).map(|_| RobotKind::Scout);
        let collectors = (0..config.num_collectors).map(|_| RobotKind::Collector);
        let robots = scouts
            .chain(collectors)
            .enumerate()
            .map(|(id, kind)| Robot::new(id, kind, map.base))
            .collect();

        Self {
            config,
            map,
            robots,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_bon_nombre_de_robots_apparait() {
        let config = SimConfig {
            num_scouts: 2,
            num_collectors: 4,
            ..SimConfig::default()
        };
        let sim = Simulation::new(config);
        assert_eq!(sim.robots.len(), 6);
    }

    #[test]
    fn tous_les_robots_demarrent_a_la_base() {
        let sim = Simulation::new(SimConfig::default());
        for robot in &sim.robots {
            assert_eq!(robot.pos, sim.map.base);
        }
    }
}
