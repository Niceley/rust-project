use crate::config::SimConfig;
use crate::resource::ResourceKind;
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
        };
        map.set(base.0, base.1, Tile::Base);
        map
    }

    #[must_use]
    pub fn generate(config: &SimConfig) -> Self {
        Self::new(config.map_width, config.map_height)
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
        assert!(!map.is_walkable(99, 99)); // hors limites
    }
}
