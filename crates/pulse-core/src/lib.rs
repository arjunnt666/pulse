//! Core types for Pulse multiplayer netcode.

pub mod error;
pub mod id;
pub mod tick;
pub mod entity;
pub mod vec3;

pub use error::{PulseError, Result};
pub use id::{ClientId, EntityId, RoomId};
pub use tick::Tick;
pub use entity::{EntityState, Component};
pub use vec3::Vec3;
