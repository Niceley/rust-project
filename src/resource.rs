#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Energy,
    Crystal,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub kind: ResourceKind,
    pub quantity: u32,
}

impl Resource {
    #[must_use]
    pub fn new(kind: ResourceKind, quantity: u32) -> Self {
        Self { kind, quantity }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.quantity == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_gisement_neuf_a_la_bonne_quantite() {
        let r = Resource::new(ResourceKind::Energy, 120);
        assert_eq!(r.kind, ResourceKind::Energy);
        assert_eq!(r.quantity, 120);
        assert!(!r.is_empty());
    }

    #[test]
    fn un_gisement_a_zero_est_vide() {
        let r = Resource::new(ResourceKind::Crystal, 0);
        assert!(r.is_empty());
    }
}
