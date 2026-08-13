//! Transport abstraction — reliable / unreliable channels.

use async_trait::async_trait;
use bytes::Bytes;
use pulse_core::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reliability { Reliable, Unreliable }

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, data: Bytes, reliability: Reliability) -> Result<()>;
    async fn recv(&self) -> Result<Option<Bytes>>;
    async fn close(&self) -> Result<()>;
}

pub struct LoopbackTransport {}

impl LoopbackTransport {
    pub fn new() -> Self { Self {} }
}

impl Default for LoopbackTransport {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl Transport for LoopbackTransport {
    async fn send(&self, _data: Bytes, _reliability: Reliability) -> Result<()> { Ok(()) }
    async fn recv(&self) -> Result<Option<Bytes>> { Ok(None) }
    async fn close(&self) -> Result<()> { Ok(()) }
}
