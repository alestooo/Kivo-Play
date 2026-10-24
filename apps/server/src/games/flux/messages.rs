use super::{
    cards::{ClashColor, FluxCard},
    state::PendingDrawKind,
};

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct FluxPublicPlayer {
    pub id: Uuid,

    pub name: String,

    pub card_count: usize,

    pub active: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct FluxPrivateState {
    pub hand: Vec<FluxCard>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingDrawKindResponse {
    DrawTwo,
    DrawFour,
}

impl From<PendingDrawKind> for PendingDrawKindResponse {
    fn from(value: PendingDrawKind) -> Self {
        match value {
            PendingDrawKind::DrawTwo => Self::DrawTwo,

            PendingDrawKind::DrawFour => Self::DrawFour,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FluxScoreResponse {
    pub player_id: Uuid,

    pub name: String,

    pub score: u16,

    pub position: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct FluxStateResponse {
    pub revision: u64,

    pub current_player_id: Uuid,

    pub direction: i8,

    pub active_color: ClashColor,

    pub discard_top: Option<FluxCard>,

    pub draw_pile_count: usize,

    pub pending_draw: u16,

    pub pending_draw_kind: Option<PendingDrawKindResponse>,

    pub clash_target_id: Option<Uuid>,

    pub clash_target_name: Option<String>,

    pub clash_serial: Option<Uuid>,

    pub round: u16,

    pub scores: Vec<FluxScoreResponse>,

    pub players: Vec<FluxPublicPlayer>,

    pub private_state: FluxPrivateState,
}
