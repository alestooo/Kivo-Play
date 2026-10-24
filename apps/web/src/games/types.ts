export type GameId =
  | 'flux'
  | 'sudorush'
  | 'stackfall'
  | 'diceforge';

export type GameStatus =
  | 'available'
  | 'coming_soon'
  | 'disabled';

export type GameEngine =
  | 'dom'
  | 'phaser'
  | 'pixi';

export type GameDefinition = {
  id: GameId;
  name: string;
  tagline: string;
  description: string;
  minPlayers: number;
  maxPlayers: number;
  supportsBots: boolean;
  status: GameStatus;
  accent: string;
  engine: GameEngine;
};
