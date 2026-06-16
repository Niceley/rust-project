use crate::config::SimConfig;
use crate::resource::{Resource, ResourceKind};
use noise::{NoiseFn, Perlin};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
pub type Position = (u16, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstacle,
    Resource(ResourceKind),
    Base,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub width: u16,
    pub height: u16,
    pub base: Position,
    tiles: Vec<Tile>,
    resources: HashMap<Position, Resource>,
}

impl Map {
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        let base = (width / 2, height / 2);
        let mut map = Self {
            width,
            height,
            base,
            tiles: vec![Tile::Empty; width as usize * height as usize],
            resources: HashMap::new(),
        };
        map.set(base.0, base.1, Tile::Base);
        map
    }

    #[must_use]
    pub fn generate(config: &SimConfig) -> Self {
        let mut map = Self::new(config.map_width, config.map_height);
        map.generate_obstacles(config);
        map.clear_around_base();
        let mut rng = StdRng::seed_from_u64(u64::from(config.seed));
        map.place_resources(config, &mut rng);
        map
    }

    fn place_resources(&mut self, config: &SimConfig, rng: &mut StdRng) {
        let mut placed = 0;
        let mut attempts = 0;
        let max_attempts = config.num_resources * 50;

        while placed < config.num_resources && attempts < max_attempts {
            attempts += 1;
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            if self.get(x, y) != Some(Tile::Empty) {
                continue;
            }

            let kind = if rng.gen_bool(0.5) {
                ResourceKind::Energy
            } else {
                ResourceKind::Crystal
            };

            let quantity: u32 = rng.gen_range(50..=200);
            self.set(x, y, Tile::Resource(kind));
            self.resources.insert((x, y), Resource::new(kind, quantity));
            placed += 1;
        }
    }

    fn generate_obstacles(&mut self, config: &SimConfig) {
        let perlin = Perlin::new(config.seed);
        let scale = 0.12;
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == self.base {
                    continue;
                }

                let value = perlin.get([f64::from(x) * scale, f64::from(y) * scale]);

                if value > config.obstacle_threshold {
                    self.set(x, y, Tile::Obstacle);
                }
            }
        }
    }

    fn clear_around_base(&mut self) {
        let (bx, by) = self.base;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                let nx = i32::from(bx) + dx;
                let ny = i32::from(by) + dy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                let (nx, ny) = (nx as u16, ny as u16);
                if self.get(nx, ny) == Some(Tile::Obstacle) {
                    self.set(nx, ny, Tile::Empty);
                }
            }
        }
    }

    fn idx(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }

    #[must_use]
    pub fn in_bounds(&self, x: u16, y: u16) -> bool {
        x < self.width && y < self.height
    }

    #[must_use]
    pub fn get(&self, x: u16, y: u16) -> Option<Tile> {
        if self.in_bounds(x, y) {
            Some(self.tiles[self.idx(x, y)])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: u16, y: u16, tile: Tile) {
        if self.in_bounds(x, y) {
            let i = self.idx(x, y);
            self.tiles[i] = tile;
        }
    }

    #[must_use]
    pub fn resource_at(&self, pos: Position) -> Option<&Resource> {
        self.resources.get(&pos)
    }

    #[must_use]
    pub fn is_walkable(&self, x: u16, y: u16) -> bool {
        matches!(
            self.get(x, y),
            Some(Tile::Empty | Tile::Resource(_) | Tile::Base)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_base_est_placee_au_centre() {
        let map = Map::new(20, 10);
        assert_eq!(map.base, (10, 5));
        assert_eq!(map.get(10, 5), Some(Tile::Base));
    }

    #[test]
    fn get_et_set_fonctionnent() {
        let mut map = Map::new(5, 5);
        assert_eq!(map.get(1, 1), Some(Tile::Empty));
        map.set(1, 1, Tile::Obstacle);
        assert_eq!(map.get(1, 1), Some(Tile::Obstacle));
    }

    #[test]
    fn hors_limites_renvoie_none() {
        let map = Map::new(5, 5);
        assert!(!map.in_bounds(5, 0));
        assert_eq!(map.get(5, 0), None);
    }

    #[test]
    fn les_obstacles_ne_sont_pas_traversables() {
        let mut map = Map::new(5, 5);
        map.set(2, 2, Tile::Obstacle);
        assert!(!map.is_walkable(2, 2));
        assert!(map.is_walkable(0, 0));
        assert!(!map.is_walkable(99, 99));
    }

    #[test]
    fn la_generation_preserve_la_base() {
        let map = Map::generate(&SimConfig::default());
        assert_eq!(map.get(map.base.0, map.base.1), Some(Tile::Base));
    }

    #[test]
    fn la_generation_place_le_bon_nombre_de_ressources() {
        let config = SimConfig::default();
        let map = Map::generate(&config);
        let count = (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (x, y)))
            .filter(|&(x, y)| matches!(map.get(x, y), Some(Tile::Resource(_))))
            .count();
        assert_eq!(count, config.num_resources);
    }

    #[test]
    fn les_ressources_ont_une_quantite_valide() {
        let map = Map::generate(&SimConfig::default());
        for resource in map.resources.values() {
            assert!((50..=200).contains(&resource.quantity));
        }
    }

    #[test]
    fn la_generation_est_deterministe_pour_une_graine() {
        let config = SimConfig::default();
        let a = Map::generate(&config);
        let b = Map::generate(&config);
        for y in 0..config.map_height {
            for x in 0..config.map_width {
                assert_eq!(a.get(x, y), b.get(x, y));
            }
        }
    }
}
