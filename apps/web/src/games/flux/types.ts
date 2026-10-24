export type ClashColor =
  | 'red'
  | 'blue'
  | 'green'
  | 'yellow';

export type ClashCardColor =
  | ClashColor
  | 'wild';

export type ClashSpecial =
  | 'draw_two'
  | 'reverse'
  | 'wild'
  | 'draw_four'
  | 'draw_six'
  | 'steal';

export type ClashCardKind =
  | 'number'
  | 'special';

export type FluxCard = {
  id: string;
  color: ClashCardColor;
  kind: ClashCardKind;
  value?: number;
  special?: ClashSpecial;
};

export type ClashPublicPlayer = {
  id: string;
  name: string;
  cardCount: number;
  active: boolean;
};

export type FluxPrivateState = {
  hand: FluxCard[];
};

export type PendingDrawKind =
  | 'draw_two'
  | 'draw_four'
  | null;

export type FluxScoreEntry = {
  playerId: string;
  name: string;
  score: number;
  position: number;
};

export type FluxClashWindow = {
  targetPlayerId: string;
  targetPlayerName: string;
  serial: string;
} | null;

export type FluxMatchState = {
  revision: number;
  currentPlayerId: string;
  direction: 1 | -1;
  activeColor: ClashColor;
  discardTop: FluxCard | null;
  drawPileCount: number;
  pendingDraw: number;
  pendingDrawKind: PendingDrawKind;
  clashWindow: FluxClashWindow;
  scores: FluxScoreEntry[];
  round: number;
  players: ClashPublicPlayer[];
  privateState: FluxPrivateState;
};

export type FluxStealOffer = {
  offerId: string;
  targetPlayerId: string;
  targetPlayerName: string;
  options: FluxCard[];
};

export type FluxRoundResult = {
  winnerId: string;
  winnerName: string;
  pointsAwarded: number;
  round: number;
  scores: FluxScoreEntry[];
  matchOver: boolean;
};
