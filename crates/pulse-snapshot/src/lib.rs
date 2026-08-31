//! World snapshots and simple delta generation.

use pulse_core::{EntityId, EntityState, Tick};
use pulse_protocol::EntityUpdate;
use std::collections::HashMap;

pub use pulse_protocol::ServerMessage;



#[derive(Debug, Clone)]
pub struct Snapshot {
    pub tick: Tick,
    pub entities: HashMap<EntityId, EntityState>,
}

impl Snapshot {
    pub fn new(tick: Tick, entities: Vec<EntityState>) -> Self {
        let map = entities.into_iter().map(|e| (e.id, e)).collect();
        Self { tick, entities: map }
    }

    pub fn to_full_message(&self) -> ServerMessage {
        ServerMessage::Snapshot {
            tick: self.tick,
            entities: self.entities.values().cloned().collect(),
            full: true,
        }
    }

    pub fn delta_from(&self, base: &Snapshot) -> ServerMessage {
        let mut updates = Vec::new();
        let mut removed = Vec::new();
        for (id, curr) in &self.entities {
            match base.entities.get(id) {
                Some(prev) => {
                    let mut u = EntityUpdate {
                        id: *id, position: None, rotation: None, velocity: None,
                    };
                    let mut changed = false;
                    if curr.position != prev.position { u.position = Some(curr.position); changed = true; }
                    if (curr.rotation - prev.rotation).abs() > 0.001 { u.rotation = Some(curr.rotation); changed = true; }
                    if curr.velocity != prev.velocity { u.velocity = Some(curr.velocity); changed = true; }
                    if changed { updates.push(u); }
                }
                None => {
                    updates.push(EntityUpdate {
                        id: *id,
                        position: Some(curr.position),
                        rotation: Some(curr.rotation),
                        velocity: Some(curr.velocity),
                    });
                }
            }
        }
        for id in base.entities.keys() {
            if !self.entities.contains_key(id) { removed.push(*id); }
        }
        ServerMessage::Delta { tick: self.tick, base_tick: base.tick, updates, removed }
    }

    pub fn interpolate(a: &Snapshot, b: &Snapshot, t: f32) -> Snapshot {
        let t = t.clamp(0.0, 1.0);
        let mut entities = HashMap::new();
        for (id, ea) in &a.entities {
            if let Some(eb) = b.entities.get(id) {
                entities.insert(*id, ea.lerp(eb, t));
            } else {
                entities.insert(*id, ea.clone());
            }
        }
        Snapshot {
            tick: if t < 1.0 { a.tick } else { b.tick },
            entities,
        }
    }
}

pub struct SnapshotBuffer {
    capacity: usize,
    history: Vec<Snapshot>,
}

impl SnapshotBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, history: Vec::with_capacity(capacity) }
    }
    pub fn push(&mut self, snap: Snapshot) {
        if self.history.len() >= self.capacity { self.history.remove(0); }
        self.history.push(snap);
    }
    pub fn get(&self, tick: Tick) -> Option<&Snapshot> {
        self.history.iter().find(|s| s.tick == tick)
    }
    pub fn latest(&self) -> Option<&Snapshot> { self.history.last() }
    pub fn previous(&self) -> Option<&Snapshot> {
        let n = self.history.len();
        if n >= 2 {
            self.history.get(n - 2)
        } else {
            None
        }
    }
    pub fn latest_delta(&self) -> Option<ServerMessage> {
        Some(self.latest()?.delta_from(self.previous()?))
    }
    pub fn len(&self) -> usize { self.history.len() }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pulse_core::{EntityId, Vec3};

    fn ent(x: f32) -> EntityState {
        EntityState {
            id: EntityId::new(),
            position: Vec3::new(x, 0.0, 0.0),
            rotation: 0.0,
            velocity: Vec3::new(1.0, 0.0, 0.0),
            components: vec![],
        }
    }

    #[test]
    fn interpolate_halfway() {
        let id = EntityId::new();
        let mut a = Snapshot::new(Tick::new(1), vec![]);
        let mut b = Snapshot::new(Tick::new(2), vec![]);
        let e0 = EntityState {
            id,
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: 0.0,
            velocity: Vec3::zero(),
            components: vec![],
        };
        let mut e1 = e0.clone();
        e1.position = Vec3::new(10.0, 0.0, 0.0);
        a.entities.insert(id, e0);
        b.entities.insert(id, e1);
        let mid = Snapshot::interpolate(&a, &b, 0.5);
        assert_eq!(mid.entities[&id].position.x, 5.0);
        let _ = ent(0.0);
    }

    #[test]
    fn delta_reports_moved_entity() {
        let id = EntityId::new();
        let mut a = Snapshot::new(Tick::new(1), vec![]);
        let mut b = Snapshot::new(Tick::new(2), vec![]);
        let e0 = EntityState {
            id,
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: 0.0,
            velocity: Vec3::zero(),
            components: vec![],
        };
        let mut e1 = e0.clone();
        e1.position = Vec3::new(3.0, 0.0, 0.0);
        a.entities.insert(id, e0);
        b.entities.insert(id, e1);
        match b.delta_from(&a) {
            ServerMessage::Delta { updates, removed, .. } => {
                assert!(removed.is_empty());
                assert_eq!(updates.len(), 1);
                assert_eq!(updates[0].position.unwrap().x, 3.0);
            }
            other => panic!("expected delta, got {other:?}"),
        }
    }
}

