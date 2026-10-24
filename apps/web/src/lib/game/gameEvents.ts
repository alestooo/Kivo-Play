export type GameEvent =
  | {
      type: 'round_started';
      round: number;
    }
  | {
      type: 'combo';
      value: number;
      label: string;
    }
  | {
      type: 'score_changed';
      playerId: string;
      score: number;
    }
  | {
      type: 'round_finished';
      winnerId: string | null;
    }
  | {
      type: 'match_finished';
      winnerId: string | null;
    };
