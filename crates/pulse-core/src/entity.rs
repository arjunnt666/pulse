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

impl EntityState {
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            id: self.id,
            position: self.position + (other.position - self.position) * t,
            rotation: self.rotation + (other.rotation - self.rotation) * t,
            velocity: self.velocity + (other.velocity - self.velocity) * t,
            components: other.components.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Component {
    Health { current: f32, max: f32 },
    Owner { client_id: crate::ClientId },
    Tag(String),
    Custom { key: String, data: Vec<u8> },
}
