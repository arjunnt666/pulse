use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tick(pub u32);

impl Tick {
    pub fn new(v: u32) -> Self { Self(v) }
    pub fn next(self) -> Self { Self(self.0.wrapping_add(1)) }
    pub fn delta(self, other: Tick) -> i32 { self.0.wrapping_sub(other.0) as i32 }
}

impl Default for Tick {
    fn default() -> Self { Self(0) }
}

impl std::fmt::Display for Tick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "t{}", self.0)
    }
}
