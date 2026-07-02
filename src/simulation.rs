use crate::config::SimConfig;
use crate::map::Map;
use crate::robot::{Robot, RobotKind};
use rand::rngs::StdRng;
use rand::SeedableRng;
use crate::base::Base;
use crate::comms::{message_channel, MessageReceiver, MessageSender};

pub struct Simulation {
    pub config: SimConfig,
    pub map: Map,
    pub robots: Vec<Robot>,
    pub base: Base,
    pub ticks: u64,
    rng: StdRng,
    tx: MessageSender,
    rx: MessageReceiver,
}

impl Simulation {
    #[must_use]
    pub fn new(config: SimConfig) -> Self {
        let map = Map::generate(&config);
        let base = Base::new(map.base);   // nouveau
        let rng = StdRng::seed_from_u64(u64::from(config.seed).wrapping_add(1));
        let (tx, rx) = message_channel();

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
            base,
            ticks: 0,
            rng,
            tx,
            rx,
        }
    }

    pub fn update(&mut self) {
        self.ticks += 1;
        for robot in &mut self.robots {
            robot.step(&self.map, &mut self.rng, &self.tx);
        }
        while let Ok(msg) = self.rx.try_recv() {
            self.base.handle_message(&msg);
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
