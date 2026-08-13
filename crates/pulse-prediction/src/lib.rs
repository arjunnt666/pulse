//! Client-side prediction and server reconciliation stubs.

use pulse_core::{EntityState, Result, Tick, Vec3};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct InputFrame {
    pub tick: Tick,
    pub move_dir: Vec3,
    pub buttons: u32,
}

#[derive(Debug, Clone)]
pub struct PredictedState {
    pub tick: Tick,
    pub position: Vec3,
    pub velocity: Vec3,
}

pub struct PredictionController {
    pending: VecDeque<InputFrame>,
    last_server: Option<EntityState>,
    predicted: PredictedState,
    move_speed: f32,
}

impl PredictionController {
    pub fn new(move_speed: f32) -> Self {
        Self {
            pending: VecDeque::new(),
            last_server: None,
            predicted: PredictedState { tick: Tick::default(), position: Vec3::zero(), velocity: Vec3::zero() },
            move_speed,
        }
    }

    pub fn apply_input(&mut self, input: InputFrame) {
        self.predicted.velocity = Vec3::new(
            input.move_dir.x * self.move_speed,
            input.move_dir.y * self.move_speed,
            input.move_dir.z * self.move_speed,
        );
        self.predicted.tick = input.tick;
        self.pending.push_back(input);
    }

    pub fn reconcile(&mut self, server_state: EntityState, server_tick: Tick) -> Result<()> {
        self.last_server = Some(server_state.clone());
        while let Some(front) = self.pending.front() {
            if front.tick.0 <= server_tick.0 { self.pending.pop_front(); } else { break; }
        }
        self.predicted.position = server_state.position;
        self.predicted.velocity = server_state.velocity;
        self.predicted.tick = server_tick;
        Ok(())
    }

    pub fn predicted_position(&self) -> Vec3 { self.predicted.position }
}
