use super::{cards::FluxCard, rules::can_play_card, state::FluxState};

use uuid::Uuid;

#[derive(Debug)]
pub enum FluxActionError {
    NotYourTurn,
    PlayerNotFound,
    CardNotFound,
    CardNotPlayable,
}

pub fn play_card(
    state: &mut FluxState,

    player_id: Uuid,

    card_id: Uuid,
) -> Result<FluxCard, FluxActionError> {
    if state.current_player_id() != Some(player_id) {
        return Err(FluxActionError::NotYourTurn);
    }

    let player_index = state
        .players
        .iter()
        .position(|player| player.player_id == player_id)
        .ok_or(FluxActionError::PlayerNotFound)?;

    let card_index = state.players[player_index]
        .hand
        .iter()
        .position(|card| card.id == card_id)
        .ok_or(FluxActionError::CardNotFound)?;

    let card = state.players[player_index].hand[card_index].clone();

    if !can_play_card(state, &card) {
        return Err(FluxActionError::CardNotPlayable);
    }

    state.players[player_index].hand.remove(card_index);

    state.discard_pile.push(card.clone());

    Ok(card)
}
