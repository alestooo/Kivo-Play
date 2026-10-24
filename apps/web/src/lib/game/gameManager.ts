import type { GameId } from '@/games/types';

export type ActiveGame = {
  id: GameId;
  startedAt: number;
};

let activeGame: ActiveGame | null = null;

export function setActiveGame(
  game: ActiveGame | null
): void {
  activeGame = game;
}

export function getActiveGame():
  ActiveGame | null {
  return activeGame;
}
