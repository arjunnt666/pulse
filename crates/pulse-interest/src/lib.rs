//! Interest / area-of-interest management.

use pulse_core::{ClientId, EntityId, EntityState, Result, Vec3};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct InterestConfig {
    pub radius: f32,
    pub max_entities: usize,
}

impl Default for InterestConfig {
    fn default() -> Self {
        Self { radius: 100.0, max_entities: 64 }
    }
}

pub struct InterestManager {
    config: InterestConfig,
    relevant: HashMap<ClientId, HashSet<EntityId>>,
}

impl InterestManager {
    pub fn new(config: InterestConfig) -> Self {
        Self { config, relevant: HashMap::new() }
    }

    pub fn compute(
        &mut self,
        client_id: ClientId,
        viewer_pos: Vec3,
        world: &[EntityState],
    ) -> Result<Vec<EntityId>> {
        let mut scored: Vec<(f32, EntityId)> = world
            .iter()
            .map(|e| (viewer_pos.distance(&e.position), e.id))
            .filter(|(d, _)| *d <= self.config.radius)
            .collect();
        scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(self.config.max_entities);
        let ids: Vec<EntityId> = scored.into_iter().map(|(_, id)| id).collect();
        self.relevant.insert(client_id, ids.iter().copied().collect());
        Ok(ids)
    }

    pub fn is_relevant(&self, client_id: &ClientId, entity_id: &EntityId) -> bool {
        self.relevant.get(client_id).map(|s| s.contains(entity_id)).unwrap_or(false)
    }
}
