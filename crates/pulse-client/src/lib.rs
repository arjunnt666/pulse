//! Client connection + prediction glue (stub).

use pulse_core::{ClientId, Result, Tick, Vec3};
use pulse_prediction::{InputFrame, PredictionController};
use tracing::info;

pub struct Client {
    pub client_id: Option<ClientId>,
    pub prediction: PredictionController,
    local_tick: Tick,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client_id: None,
            prediction: PredictionController::new(5.0),
            local_tick: Tick::default(),
        }
    }

    pub fn on_welcome(&mut self, id: ClientId) {
        info!(%id, "welcomed by server");
        self.client_id = Some(id);
    }

    pub fn send_input(&mut self, move_dir: Vec3, buttons: u32) {
        self.local_tick = self.local_tick.next();
        let frame = InputFrame { tick: self.local_tick, move_dir, buttons };
        self.prediction.apply_input(frame);
    }

    pub fn predicted_pos(&self) -> Vec3 {
        self.prediction.predicted_position()
    }
}

impl Default for Client {
    fn default() -> Self { Self::new() }
}
