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
  total: number;
  complete: boolean;
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
  players: Player[];
};

export type RoomSummary = {
  room_id: string;
  room_name: string;
  host_name: string;
  visibility: 'public' | 'private';
  player_count: number;
  max_players: number;
  joinable: boolean;
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
      type: 'set_ready';
      ready: boolean;
    }
  | {
      type: 'start_game';
    }
  | {
      type: 'roll_turn_die';
      die_index: number;
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
      type: 'turn_order_state';
      players: TurnOrderPlayer[];
      all_complete: boolean;
    }
  | {
      type: 'turn_order_finished';
      ranking: RankingEntry[];
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