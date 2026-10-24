use super::{
    cards::{FluxCard, FluxCardKind, FluxSpecial},
    state::{FluxState, PendingDrawKind},
};

pub fn can_play_card(state: &FluxState, card: &FluxCard) -> bool {
    if state.pending_steal.is_some() {
        return false;
    }

    if let Some(pending_kind) = state.pending_draw_kind {
        return matches!(
            (pending_kind, &card.kind),
            (
                PendingDrawKind::DrawTwo,
                FluxCardKind::Special {
                    special: FluxSpecial::DrawTwo,
                }
            ) | (
                PendingDrawKind::DrawFour,
                FluxCardKind::Special {
                    special: FluxSpecial::DrawFour,
                }
            )
        );
    }

    match &card.kind {
        FluxCardKind::Special {
            special: FluxSpecial::Wild | FluxSpecial::DrawFour | FluxSpecial::DrawSix,
        } => {
            return true;
        }

        _ => {}
    }

    if card.color.normal_color() == Some(state.active_color) {
        return true;
    }

    let Some(top) = state.discard_pile.last() else {
        return true;
    };

    match (&card.kind, &top.kind) {
        (FluxCardKind::Number { value: left }, FluxCardKind::Number { value: right }) => {
            left == right
        }

        (FluxCardKind::Special { special: left }, FluxCardKind::Special { special: right }) => {
            left == right
        }

        _ => false,
    }
}
