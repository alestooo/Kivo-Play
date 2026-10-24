import type {
  ClashColor,
  FluxCard,
  FluxMatchState,
  FluxRoundResult,
  FluxScoreEntry,
  FluxStealOffer,
  PendingDrawKind
} from '@/games/flux/types';

import type {
  GameId
} from '@/games/types';

export type Player = {
  id: string;
  name: string;
  is_host: boolean;
  ready: boolean;
};

export type TurnOrderPlayer = {
  id: string;
  name: string;
  rolls: Array<number | null>;
  rolled: boolean[];
  total: number;
  complete: boolean;
  roll_count: number;
};

export type DiePose = {
  owner_id: string;
  die_index: number;
  x: number;
  y: number;
  rotation: number;
};

export type PhysicsFrame = {
  dice: DiePose[];
};

export type RankingEntry = {
  player_id: string;
  name: string;
  total: number;
  position: number;
  tied: boolean;
};

export type RoomStatePayload = {
  room_code: string;
  room_name: string;
  host_id: string;
  max_players: number;
  visibility: string;
  state: string;
  selected_game: GameId | null;
  players: Player[];
};

export type RoomSummary = {
  room_id: string;
  room_name: string;
  host_name: string;
  visibility:
    | 'public'
    | 'private';
  player_count: number;
  max_players: number;
  joinable: boolean;
};

export type FluxPublicPlayerPayload = {
  id: string;
  name: string;
  card_count: number;
  active: boolean;
};

export type FluxScorePayload = {
  player_id: string;
  name: string;
  score: number;
  position: number;
};

export type FluxStatePayload = {
  revision: number;
  current_player_id: string;
  direction: number;
  active_color: ClashColor;
  discard_top: FluxCard | null;
  draw_pile_count: number;
  pending_draw: number;
  pending_draw_kind: PendingDrawKind;
  clash_target_id:
    string | null;
  clash_target_name:
    string | null;
  clash_serial:
    string | null;
  round: number;
  scores:
    FluxScorePayload[];
  players:
    FluxPublicPlayerPayload[];
  private_state: {
    hand: FluxCard[];
  };
};

export type ClientMessage =
  | {
      type: 'ping';
    }
  | {
      type: 'create_room';
      player_name: string;
      room_name: string;
      max_players: number;
      visibility: string;
    }
  | {
      type: 'join_room';
      player_name: string;
      room_code: string;
    }
  | {
      type: 'join_public_room';
      player_name: string;
      room_id: string;
    }
  | {
      type: 'list_rooms';
      page: number;
      search: string;
    }
  | {
      type: 'select_game';
      game_id: GameId;
    }
  | {
      type: 'set_ready';
      ready: boolean;
    }
  | {
      type: 'start_game';
    }
  | {
      type: 'roll_turn_die';
      die_index: number;
    }
  | {
      type: 'flux_play_card';
      card_id: string;
      chosen_color:
        ClashColor | null;
      target_player_id:
        string | null;
    }
  | {
      type: 'flux_begin_steal';
      card_id: string;
      target_player_id: string;
    }
  | {
      type: 'flux_choose_steal';
      offer_id: string;
      card_id: string;
    }
  | {
      type: 'flux_clash';
    }
  | {
      type: 'flux_round_ready';
      ready: boolean;
    }
  | {
      type: 'flux_next_round';
    }
  | {
      type: 'flux_play_again';
    }
  | {
      type: 'flux_draw';
    };

export type ServerMessage =
  | {
      type: 'pong';
    }
  | ({
      type: 'room_created';
      player_id: string;
    } & RoomStatePayload)
  | ({
      type: 'room_joined';
      player_id: string;
    } & RoomStatePayload)
  | ({
      type: 'room_state';
    } & RoomStatePayload)
  | {
      type: 'room_directory';
      rooms: RoomSummary[];
      page: number;
      total_pages: number;
      total_rooms: number;
    }
  | {
      type: 'turn_order_started';
      room_code: string;
      players: TurnOrderPlayer[];
    }
  | {
      type: 'turn_order_physics';
      frames: PhysicsFrame[];
    }
  | {
      type: 'turn_order_state';
      players: TurnOrderPlayer[];
      all_complete: boolean;
    }
  | {
      type: 'turn_order_finished';
      ranking: RankingEntry[];
    }
  | {
      type: 'flux_state';
      state: FluxStatePayload;
      playable_card_ids: string[];
    }
  | {
      type: 'flux_card_played';
      card: FluxCard;
      revision: number;
    }
  | {
      type: 'flux_steal_offer';
      offer_id: string;
      target_player_id: string;
      target_player_name: string;
      options: FluxCard[];
    }
  | {
      type: 'flux_round_ready_state';
      ready_player_ids: string[];
      all_ready: boolean;
    }
  | {
      type: 'flux_impact';
      text: string;
    }
  | {
      type: 'flux_round_finished';
      winner_id: string;
      winner_name: string;
      points_awarded: number;
      round: number;
      scores: FluxScorePayload[];
      match_over: boolean;
    }
  | {
      type: 'error';
      message: string;
    };

export type ConnectionStatus =
  | 'disconnected'
  | 'connecting'
  | 'connected'
  | 'error';

function mapScores(
  scores:
    FluxScorePayload[]
): FluxScoreEntry[] {
  return scores.map(
    (score) => ({
      playerId:
        score.player_id,
      name:
        score.name,
      score:
        score.score,
      position:
        score.position
    })
  );
}

export function mapFluxState(
  payload:
    FluxStatePayload
): FluxMatchState {
  return {
    revision:
      payload.revision,

    currentPlayerId:
      payload.current_player_id,

    direction:
      payload.direction >= 0
        ? 1
        : -1,

    activeColor:
      payload.active_color,

    discardTop:
      payload.discard_top,

    drawPileCount:
      payload.draw_pile_count,

    pendingDraw:
      payload.pending_draw,

    pendingDrawKind:
      payload.pending_draw_kind,

    clashWindow:
      payload.clash_target_id &&
      payload.clash_target_name &&
      payload.clash_serial
        ? {
            targetPlayerId:
              payload.clash_target_id,
            targetPlayerName:
              payload.clash_target_name,
            serial:
              payload.clash_serial
          }
        : null,

    scores:
      mapScores(
        payload.scores
      ),

    round:
      payload.round,

    players:
      payload.players.map(
        (player) => ({
          id: player.id,
          name: player.name,
          cardCount:
            player.card_count,
          active:
            player.active
        })
      ),

    privateState: {
      hand:
        payload.private_state.hand
    }
  };
}

export function mapFluxStealOffer(
  message:
    Extract<
      ServerMessage,
      {
        type:
          'flux_steal_offer'
      }
    >
): FluxStealOffer {
  return {
    offerId:
      message.offer_id,
    targetPlayerId:
      message.target_player_id,
    targetPlayerName:
      message.target_player_name,
    options:
      message.options
  };
}

export function mapFluxRoundResult(
  message:
    Extract<
      ServerMessage,
      {
        type:
          'flux_round_finished'
      }
    >
): FluxRoundResult {
  return {
    winnerId:
      message.winner_id,
    winnerName:
      message.winner_name,
    pointsAwarded:
      message.points_awarded,
    round:
      message.round,
    scores:
      mapScores(
        message.scores
      ),
    matchOver:
      message.match_over
  };
}
