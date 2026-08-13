//! World snapshots and simple delta generation.

use pulse_core::{EntityId, EntityState, Tick};
use pulse_protocol::{EntityUpdate, ServerMessage};
use std::collections::HashMap;

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
}
