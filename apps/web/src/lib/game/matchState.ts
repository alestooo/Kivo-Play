export type MatchPhase =
  | 'intro'
  | 'playing'
  | 'round_result'
  | 'match_result';

export type MatchState = {
  phase: MatchPhase;
  round: number;
};
