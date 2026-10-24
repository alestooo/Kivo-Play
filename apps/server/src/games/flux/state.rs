use std::time::Instant;

use super::cards::{ClashColor, FluxCard};

use uuid::Uuid;

pub const STARTING_HAND_SIZE: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingDrawKind {
    DrawTwo,
    DrawFour,
}

#[derive(Debug, Clone)]
pub struct FluxPlayerState {
    pub player_id: Uuid,

    pub hand: Vec<FluxCard>,
}

#[derive(Debug, Clone)]
pub struct PendingStealOffer {
    pub offer_id: Uuid,

    pub attacker_id: Uuid,

    pub target_player_id: Uuid,

    pub steal_card_id: Uuid,

    pub option_card_ids: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct PendingClash {
    pub target_player_id: Uuid,

    pub serial: Uuid,

    pub opened_at: Instant,
}

#[derive(Debug, Clone)]
pub struct FluxState {
    pub players: Vec<FluxPlayerState>,

    pub draw_pile: Vec<FluxCard>,

    pub discard_pile: Vec<FluxCard>,

    /*
     * Explicit authoritative card shown in the centre.
     * This is updated only when the server accepts a card play.
     */
    pub last_played_card: Option<FluxCard>,

    /*
     * Monotonic state revision. Clients ignore older revisions.
     */
    pub revision: u64,

    pub current_player_index: usize,

    pub direction: i8,

    pub active_color: ClashColor,

    pub pending_draw: u16,

    pub pending_draw_kind: Option<PendingDrawKind>,

    pub pending_steal: Option<PendingStealOffer>,

    pub pending_clash: Option<PendingClash>,

    pub round: u16,
}

impl FluxState {
    pub fn current_player_id(&self) -> Option<Uuid> {
        self.players
            .get(self.current_player_index)
            .map(|player| player.player_id)
    }

    pub fn bump_revision(&mut self) {
        self.revision = self.revision.saturating_add(1);
    }
}
