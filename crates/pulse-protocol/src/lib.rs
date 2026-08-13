//! Wire protocol messages.

use pulse_core::{ClientId, EntityId, EntityState, Tick, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Join { room: String, name: String },
    Leave,
    Input { tick: Tick, move_dir: Vec3, buttons: u32 },
    Ping { client_time: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Welcome { client_id: ClientId, tick_rate: u32 },
    Snapshot { tick: Tick, entities: Vec<EntityState>, full: bool },
    Delta { tick: Tick, base_tick: Tick, updates: Vec<EntityUpdate>, removed: Vec<EntityId> },
    Pong { client_time: u64, server_time: u64 },
    Kick { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityUpdate {
    pub id: EntityId,
    pub position: Option<Vec3>,
    pub rotation: Option<f32>,
    pub velocity: Option<Vec3>,
}
