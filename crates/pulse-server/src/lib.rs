//! Authoritative in-process server tick.

use pulse_core::{ClientId, EntityState, Result, Tick};
use pulse_interest::{InterestConfig, InterestManager};
use pulse_snapshot::{Snapshot, SnapshotBuffer};
use std::collections::HashMap;
use tracing::info;

pub struct Server {
    tick: Tick,
    tick_rate: u32,
    clients: HashMap<ClientId, ClientSlot>,
    entities: Vec<EntityState>,
    interest: InterestManager,
    snapshots: SnapshotBuffer,
}

struct ClientSlot {
    name: String,
}

impl Server {
    pub fn new(tick_rate: u32) -> Self {
        Self {
            tick: Tick::default(),
            tick_rate,
            clients: HashMap::new(),
            entities: Vec::new(),
            interest: InterestManager::new(InterestConfig::default()),
            snapshots: SnapshotBuffer::new(128),
        }
    }

    pub fn add_client(&mut self, id: ClientId, name: String) {
        info!(%id, %name, "client joined");
        self.clients.insert(id, ClientSlot { name });
    }

    pub fn spawn(&mut self, entity: EntityState) {
        self.entities.push(entity);
    }

    pub fn tick(&mut self) -> Result<()> {
        let dt = 1.0 / (self.tick_rate.max(1) as f32);
        for e in &mut self.entities {
            e.position = e.position + e.velocity * dt;
        }
        self.tick = self.tick.next();
        let snap = Snapshot::new(self.tick, self.entities.clone());
        self.snapshots.push(snap);
        Ok(())
    }

    pub fn current_tick(&self) -> Tick {
        self.tick
    }

    pub fn latest_snapshot(&self) -> Option<&Snapshot> {
        self.snapshots.latest()
    }
}
