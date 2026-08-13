use serde::{Deserialize, Serialize};
use crate::id::EntityId;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub id: EntityId,
    pub position: Vec3,
    pub rotation: f32,
    pub velocity: Vec3,
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Component {
    Health { current: f32, max: f32 },
    Owner { client_id: crate::ClientId },
    Tag(String),
    Custom { key: String, data: Vec<u8> },
}
