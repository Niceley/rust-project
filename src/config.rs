use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SimConfig {
    pub map_width: u16,
    pub map_height: u16,
    pub num_scouts: usize,
    pub num_collectors: usize,
    pub tick_rate: Duration,
    pub seed: u32,
    pub obstacle_threshold: f64,
    pub num_resources: usize,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            map_width: 60,
            map_height: 30,
            num_scouts: 3,
            num_collectors: 3,
            tick_rate: Duration::from_millis(120),
            seed: 42,
            obstacle_threshold: 0.35,
            num_resources: 12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_config_par_defaut_est_coherente() {
        let config = SimConfig::default();
        assert!(config.map_width > 0);
        assert!(config.map_height > 0);
        assert!(config.num_scouts + config.num_collectors > 0);
    }
}
