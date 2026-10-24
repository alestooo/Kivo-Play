pub mod physics;

use serde::Serialize;
use uuid::Uuid;

pub const DICE_PER_PLAYER: usize = 3;

#[derive(Debug, Clone, Copy)]
pub struct DieState {
    pub rolled: bool,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub value: Option<u8>,
}

impl Default for DieState {
    fn default() -> Self {
        Self {
            rolled: false,
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            value: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhysicsDieInput {
    pub owner_id: Uuid,
    pub die_index: usize,
    pub state: DieState,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiePoseResponse {
    pub owner_id: Uuid,
    pub die_index: usize,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PhysicsFrameResponse {
    pub dice: Vec<DiePoseResponse>,
}

#[derive(Debug, Clone)]
pub struct PhysicsSimulation {
    pub frames: Vec<PhysicsFrameResponse>,
    pub dice: Vec<PhysicsDieInput>,
}
